//----------------------------------------------------------------------------------------------------------------------
// <auto-generated />
// This code was generated by the Microsoft.Quantum.Qir.Runtime.Tools package.
// The purpose of this source code file is to provide an entry-point for executing a QIR program.
// It handles parsing of command line arguments, and it invokes an entry-point function exposed by the QIR program.
//----------------------------------------------------------------------------------------------------------------------

#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <vector>

#include "CLI11.hpp"

#include "QirRuntime.hpp"
#include "QirContext.hpp"

#include "SimFactory.hpp"

using namespace Microsoft::Quantum;
using namespace std;

// Auxiliary functions for interop with Q# Bool type.
const char InteropFalseAsChar = 0x0;
const char InteropTrueAsChar = 0x1;
map<string, bool> BoolAsCharMap{
    {"0", InteropFalseAsChar},
    {"false", InteropFalseAsChar},
    {"1", InteropTrueAsChar},
    {"true", InteropTrueAsChar}
};

extern "C" void UseBoolArg(
    char BoolArg
); // QIR interop function.

int main(int argc, char* argv[])
{
    CLI::App app("QIR Standalone Entry Point");

    // Initialize runtime.
    unique_ptr<IRuntimeDriver> sim = CreateFullstateSimulator();
    QirContextScope qirctx(sim.get(), false /*trackAllocatedObjects*/);

    // Add a command line option for each entry-point parameter.
    char BoolArgCli;
    BoolArgCli = InteropFalseAsChar;
    app.add_option("--BoolArg", BoolArgCli, "Option to provide a value for the BoolArg parameter")
        ->required()
        ->transform(CLI::CheckedTransformer(BoolAsCharMap, CLI::ignore_case));

    // After all the options have been added, parse arguments from the command line.
    CLI11_PARSE(app, argc, argv);

    // Cast parsed arguments to its interop types.
    char BoolArgInterop = BoolArgCli;

    // Execute the entry point operation.
    UseBoolArg(
        BoolArgInterop
    );

    // Flush the output of the simulation.
    cout.flush();

    return 0;
}
