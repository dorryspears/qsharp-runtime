﻿// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

namespace Microsoft.Quantum.Diagnostics {

    /// # Summary
    /// Compiler-recognized attribute used to mark a unit test.
    /// 
    /// # Input
    /// ## ExecutionTarget
    /// The name of the target to execute the test on. 
    /// The name has to be either one of the known targets, or a fully qualified name.
    /// Known targets are: QuantumSimulator, ToffoliSimulator.
    ///
    /// # Example
    /// The following is a unit test that checks if `2 + 3` is `5`:
    /// ```qsharp
    /// @Test("QuantumSimulator")
    /// function AdditionIsCorrect() : Unit {
    ///     EqualityFactI(2 + 3, 5, "Addition did not work correctly.");
    /// }
    /// ```
    @Attribute()
    newtype Test = (ExecutionTarget : String);

    /// # Summary
    /// Compiler-recognized attribute via which an alternative name can be defined 
    /// that may be used when loading a type or callable for testing purposes.
    /// 
    /// # Input
    /// Defined name for testing purposes. 
    /// The String is expected to contain a fully qualified name. 
    @Attribute()
    newtype EnableTestingViaName = String;
}


