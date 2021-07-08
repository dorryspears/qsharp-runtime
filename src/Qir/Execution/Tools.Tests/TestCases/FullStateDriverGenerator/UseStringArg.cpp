//----------------------------------------------------------------------------------------------------------------------
// <auto-generated />
// This code was generated by the Microsoft.Quantum.Qir.Tools package.
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

// Auxiliary functions for interop with Q# String type.
const char* TranslateStringToCharBuffer(string& s)
{
    return s.c_str();
}

extern "C" void UseStringArg(
    const char* StringArg
); // QIR interop function.

int main(int argc, char* argv[])
{
    CLI::App app("QIR Standalone Entry Point");

    // Initialize runtime.
    unique_ptr<IRuntimeDriver> sim = CreateFullstateSimulator();
    QirContextScope qirctx(sim.get(), false /*trackAllocatedObjects*/);

    // Add the --simulation-output option.
    string simulationOutputFile;
    CLI::Option* simulationOutputFileOpt = app.add_option(
        "--simulation-output",
        simulationOutputFile,
        "File where the output produced during the simulation is written");

    // Add a command line option for each entry-point parameter.
    string StringArgCli;
    app.add_option("--StringArg", StringArgCli, "Option to provide a value for the StringArg parameter")
        ->required();

    // After all the options have been added, parse arguments from the command line.
    CLI11_PARSE(app, argc, argv);

    // Cast parsed arguments to its interop types.
    const char* StringArgInterop = TranslateStringToCharBuffer(StringArgCli);

    // Redirect the simulator output from std::cout if the --simulation-output option is present.
    ostream* simulatorOutputStream = &cout;
    ofstream simulationOutputFileStream;
    if (!simulationOutputFileOpt->empty())
    {
        simulationOutputFileStream.open(simulationOutputFile);
        SetOutputStream(simulationOutputFileStream);
        simulatorOutputStream = &simulationOutputFileStream;
    }

    // Execute the entry point operation.
    UseStringArg(
        StringArgInterop
    );

    // Flush the output of the simulation.
    simulatorOutputStream->flush();
    if (simulationOutputFileStream.is_open())
    {
        simulationOutputFileStream.close();
    }

    return 0;
}
