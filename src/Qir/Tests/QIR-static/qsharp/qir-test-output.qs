// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

namespace Microsoft.Quantum.Testing.QIR.Out {

    open Microsoft.Quantum.Intrinsic;

    @EntryPoint()
    function MessageTest(msg: String) : Unit {
        Message(msg);
    }

    @EntryPoint()
    function EmptyMessageTest() : Unit {
        Message("");
    }

} // namespace Microsoft.Quantum.Testing.QIR.Out

