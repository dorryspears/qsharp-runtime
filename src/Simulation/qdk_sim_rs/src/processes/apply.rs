// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Public implementations and crate-private functions for applying processes
//! in each different representation.

use cauchy::c64;
use itertools::Itertools;
use ndarray::{Array, Array2, Array3, ArrayView2, Axis};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

use crate::{
    chp_decompositions::ChpOperation,
    error::QdkSimError,
    linalg::ConjBy,
    log, log_as_err, Pauli, Process,
    ProcessData::*,
    State,
    StateData::{self, *},
    Tableau, VariantName, C64,
};

use super::promote_pauli_channel;

impl Process {
    /// Applies this process to a quantum register with a given
    /// state, returning the new state of that register.
    pub fn apply(&self, state: &State) -> Result<State, QdkSimError> {
        if state.n_qubits != self.n_qubits {
            return Err(QdkSimError::WrongNumberOfQubits {
                expected: self.n_qubits,
                actual: state.n_qubits,
            });
        }

        match &self.data {
            Unitary(u) => apply_unitary(u, state),
            KrausDecomposition(ks) => apply_kraus_decomposition(ks, state),
            MixedPauli(paulis) => apply_pauli_channel(paulis, state),
            Sequence(processes) => {
                // TODO[perf]: eliminate the extraneous clone here.
                let mut acc_state = state.clone();
                for process in processes {
                    acc_state = process.apply(state)?;
                }
                Ok(acc_state)
            }
            // TODO: Support applying CHP decompositions and superoperators to
            //       entire registers. Currently only supported acting on
            //       subregisters via [`apply_to`].
            Superoperator(_) => todo!(),
            ChpDecomposition(_operations) => todo!(),
            Unsupported => Err(QdkSimError::UnsupportedApply {
                channel_variant: self.variant_name(),
                state_variant: state.variant_name(),
            }),
        }
    }

    /// Applies this process to the given qubits in a register with a given
    /// state, returning the new state of that register.
    pub fn apply_to(&self, idx_qubits: &[usize], state: &State) -> Result<State, QdkSimError> {
        // If we have a sequence, we can apply each in turn and exit early.
        if let Sequence(channels) = &self.data {
            // TODO[perf]: eliminate the extraneous clone here.
            let mut acc_state = state.clone();
            for channel in channels {
                acc_state = channel.apply_to(idx_qubits, &acc_state)?;
            }
            return Ok(acc_state);
        }

        // Fail if there's not enough qubits.
        if state.n_qubits < self.n_qubits {
            return log_as_err(format!(
                "Channel acts on {} qubits, but a state on only {} qubits was given.\n\nChannel:\n{:?}\n\nState:\n{:?}",
                self.n_qubits, state.n_qubits, self, state
            ));
        }

        // Fail if any indices are repeated.
        if idx_qubits.iter().unique().count() < idx_qubits.len() {
            return log_as_err(format!(
                "List of qubit indices {:?} contained repeated elements.",
                idx_qubits
            ));
        }

        // Make sure that there are only as many indices as qubits that this
        // channel acts upon.
        if idx_qubits.len() != self.n_qubits {
            return log_as_err(format!(
                "Qubit indices were specified as {:?}, but this channel only acts on {} qubits.",
                idx_qubits, self.n_qubits
            ));
        }

        // At this point we know that idx_qubits has self.n_qubits many unique
        // indices, such that we can meaningfully apply the channel to the
        // qubits described by idx_qubits.
        //
        // To do so in general, we can proceed to make a new channel
        // that expands this channel to act on the full register and then use
        // the ordinary apply method.
        //
        // In some cases, however, we can do so more efficiently by working
        // with the small channel directly, so we check for those cases first
        // before falling through to the general case.

        // TODO[perf]: For larger systems, we could add another "fast path" using
        //             matrix multiplication kernels to avoid extending
        //             channels to larger Hilbert spaces.
        //             For smaller systems, extending channels and possibly
        //             caching them is likely to be more performant; need to
        //             tune to find crossover point.
        if let ChpDecomposition(operations) = &self.data {
            if let Stabilizer(tableau) = &state.data {
                return apply_chp_decomposition_to(operations, state.n_qubits, idx_qubits, tableau);
            }
        }

        if let Superoperator(mtx) = &self.data {
            return match &state.data {
                StateData::Mixed(rho) => Ok(State {
                    n_qubits: state.n_qubits,
                    data: StateData::Mixed(apply_superoperator_to_density_matrix(
                        state.n_qubits,
                        idx_qubits,
                        mtx.view(),
                        rho.view(),
                    )?),
                }),
                _ => Err(QdkSimError::UnsupportedApply {
                    channel_variant: self.variant_name(),
                    state_variant: state.variant_name(),
                }),
            };
        }
        // TODO: Add superoperator case here, since that will let us avoid
        //       calling extend_k_to_n.

        // Having tried fast paths above, we now fall back to the most general
        // case.
        match self.n_qubits {
            1 => {
                if state.n_qubits == 1 {
                    self.apply(state)
                } else {
                    self.extend_one_to_n(idx_qubits[0], state.n_qubits)
                        .apply(state)
                }
            }
            // TODO[perf]: If the size of the register matches the size of the
            //             channel, permute rather than expanding.
            2 => self
                .extend_two_to_n(idx_qubits[0], idx_qubits[1], state.n_qubits)
                .apply(state),
            _ => {
                log(&format!(
                    "Expanding {}-qubit channels is not yet implemented.",
                    self.n_qubits
                ));
                unimplemented!("");
            }
        }
    }
}

fn apply_superoperator_to_density_matrix<'a, O, M>(
    n_qubits: usize,
    idx_qubits: &[usize],
    superop: O,
    state: M,
) -> Result<Array2<c64>, QdkSimError>
where
    O: Into<ArrayView2<'a, c64>>,
    M: Into<ArrayView2<'a, c64>>,
{
    let superop: ArrayView2<c64> = superop.into();
    let state: ArrayView2<c64> = state.into();
    match idx_qubits.len() {
        1 => {
            let idx_qubit = idx_qubits[0];
            if superop.shape() != &[4, 4] {
                Err(QdkSimError::MiscError(format!(
                    "Expected 4x4 superoperator but got {:?}.",
                    superop.shape()
                )))?;
            }
            let superop = superop.into_shape((2, 2, 2, 2)).unwrap();
            // In the column-stacking basis, `superop` is indexed as
            // `rho_out[i, j] = sum_{k, l} S[j, i, l, k] rho_in[k, l]`.
            // For applying to multiple qubits, we'll represent this by
            // reshaping such that:
            // `rho_out[:, i, :, :, j, :] = sum_{k, l} S[j, i, l, k] rho_in[:, k, :, :, l, :]`.
            let n_left = idx_qubit;
            let n_right = n_qubits - idx_qubit - 1;
            let rho_in = state
                .into_shape((
                    2usize.pow(n_left as u32),
                    2,
                    2usize.pow(n_right as u32),
                    2usize.pow(n_left as u32),
                    2,
                    2usize.pow(n_right as u32),
                ))
                .map_err(|e| QdkSimError::InternalShapeError(e))?;
            let mut rho_out = rho_in.to_owned();
            for i in 0..2usize {
                for j in 0..2usize {
                    for k in 0..2usize {
                        for l in 0..2usize {
                            rho_out.slice_mut(s![.., i, .., .., j, ..]).assign(
                                &(superop[(j, i, k, l)] * &rho_in.slice(s![.., k, .., .., l, ..])),
                            );
                        }
                    }
                }
            }
            let shape = state.shape();
            let rho_out = rho_out.into_shape((shape[0], shape[1])).unwrap();
            Ok(rho_out)
        }
        _ => Err(QdkSimError::NotYetImplemented(
            "Superoperators can currently only be applied to one-qubit density operators."
                .to_string(),
        )),
    }
}

fn apply_chp_decomposition_to(
    operations: &[ChpOperation],
    n_qubits: usize,
    idx_qubits: &[usize],
    tableau: &Tableau,
) -> Result<State, QdkSimError> {
    let mut new_tableau = tableau.clone();
    for operation in operations {
        match *operation {
            ChpOperation::Phase(idx) => new_tableau.apply_s_mut(idx_qubits[idx]),
            ChpOperation::AdjointPhase(idx) => new_tableau.apply_s_adj_mut(idx_qubits[idx]),
            ChpOperation::Hadamard(idx) => new_tableau.apply_h_mut(idx_qubits[idx]),
            ChpOperation::Cnot(idx_control, idx_target) => {
                new_tableau.apply_cnot_mut(idx_qubits[idx_control], idx_qubits[idx_target])
            }
        };
    }
    Ok(State {
        n_qubits,
        data: Stabilizer(new_tableau),
    })
}

fn apply_unitary(u: &Array2<C64>, state: &State) -> Result<State, QdkSimError> {
    Ok(State {
        n_qubits: state.n_qubits,
        data: match &state.data {
            Pure(psi) => Pure(u.dot(psi)),
            Mixed(rho) => Mixed(rho.conjugate_by(&u.into())),
            Stabilizer(_tableau) => {
                return Err(QdkSimError::NotYetImplemented(
                    "TODO: Promote stabilizer state to state vector and recurse.".to_string(),
                ))
            }
        },
    })
}

fn apply_kraus_decomposition(ks: &Array3<C64>, state: &State) -> Result<State, QdkSimError> {
    Ok(State {
        n_qubits: state.n_qubits,
        data: match &state.data {
            Pure(psi) => {
                // We can't apply a channel with more than one Kraus operator (Choi rank > 1) to a
                // pure state directly, so if the Choi rank is bigger than 1, promote to
                // Mixed and recurse.
                if ks.shape()[0] == 1 {
                    Pure({
                        let k: ArrayView2<C64> = ks.slice(s![0, .., ..]);
                        k.dot(psi)
                    })
                } else {
                    apply_kraus_decomposition(ks, &state.to_mixed())?.data
                }
            }
            Mixed(rho) => Mixed({
                let mut sum: Array2<C64> = Array::zeros((rho.shape()[0], rho.shape()[1]));
                for k in ks.axis_iter(Axis(0)) {
                    sum = sum + rho.conjugate_by(&k);
                }
                sum
            }),
            Stabilizer(_tableau) => {
                return Err(QdkSimError::NotYetImplemented(
                    "TODO: Promote stabilizer state to state vector and recurse.".to_string(),
                ))
            }
        },
    })
}

fn apply_pauli_channel(paulis: &[(f64, Vec<Pauli>)], state: &State) -> Result<State, QdkSimError> {
    Ok(State {
        n_qubits: state.n_qubits,
        data: match &state.data {
            Pure(_) | Mixed(_) => {
                // Promote and recurse.
                let promoted = promote_pauli_channel(paulis)?;
                return promoted.apply(state);
            }
            Stabilizer(tableau) => {
                // TODO[perf]: Introduce an apply_mut method to
                //             avoid extraneous cloning.
                let mut new_tableau = tableau.clone();
                // Sample a Pauli and apply it.
                let weighted = WeightedIndex::new(paulis.iter().map(|(pr, _)| pr)).unwrap();
                let idx = weighted.sample(&mut thread_rng());
                let pauli = &paulis[idx].1;
                // TODO: Consider moving the following to a method
                //       on Tableau itself.
                for (idx_qubit, p) in pauli.iter().enumerate() {
                    match p {
                        Pauli::I => (),
                        Pauli::X => new_tableau.apply_x_mut(idx_qubit),
                        Pauli::Y => new_tableau.apply_y_mut(idx_qubit),
                        Pauli::Z => new_tableau.apply_z_mut(idx_qubit),
                    }
                }
                Stabilizer(new_tableau)
            }
        },
    })
}
