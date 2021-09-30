
//------------------------------------------------------------------------------
// This code was generated by a tool.
//
//   Tool : Bond Compiler 0.12.1.0
//   Input filename:  BondSchemas\Execution.bond
//   Output filename: Execution_types.cs
//
// Changes to this file may cause incorrect behavior and will be lost when
// the code is regenerated.
// <auto-generated />
//------------------------------------------------------------------------------


// suppress "Missing XML comment for publicly visible type or member"
#pragma warning disable 1591


#region ReSharper warnings
// ReSharper disable PartialTypeWithSinglePart
// ReSharper disable RedundantNameQualifier
// ReSharper disable InconsistentNaming
// ReSharper disable CheckNamespace
// ReSharper disable UnusedParameter.Local
// ReSharper disable RedundantUsingDirective
#endregion

namespace Microsoft.Quantum.Qir.Serialization
{
    using System.Collections.Generic;

    [System.CodeDom.Compiler.GeneratedCode("gbc", "0.12.1.0")]
    public enum DataType
    {
        Integer,
        Double,
        BytePointer,
        Collection,
    }

    [global::Bond.Schema]
    [System.CodeDom.Compiler.GeneratedCode("gbc", "0.12.1.0")]
    public partial class Parameter
    {
        [global::Bond.Id(5)]
        public string Name { get; set; }

        [global::Bond.Id(10)]
        public int Position { get; set; }

        [global::Bond.Id(15)]
        public DataType Type { get; set; }

        [global::Bond.Id(20), global::Bond.Type(typeof(global::Bond.Tag.nullable<List<DataType>>))]
        public List<DataType> ElementTypes { get; set; }

        public Parameter()
            : this("Microsoft.Quantum.Qir.Serialization.Parameter", "Parameter")
        { }

        protected Parameter(string fullName, string name)
        {
            Name = "";
            Type = DataType.BytePointer;
        }
    }

    [global::Bond.Schema]
    [System.CodeDom.Compiler.GeneratedCode("gbc", "0.12.1.0")]
    public partial class ArgumentValue
    {
        [global::Bond.Id(5)]
        public DataType Type { get; set; }

        [global::Bond.Id(10), global::Bond.Type(typeof(global::Bond.Tag.nullable<long>))]
        public long? Integer { get; set; }

        [global::Bond.Id(15), global::Bond.Type(typeof(global::Bond.Tag.nullable<double>))]
        public double? Double { get; set; }

        [global::Bond.Id(20), global::Bond.Type(typeof(global::Bond.Tag.nullable<List<sbyte>>))]
        public List<sbyte> BytePointer { get; set; }

        [global::Bond.Id(25), global::Bond.Type(typeof(global::Bond.Tag.nullable<List<ArgumentValue>>))]
        public List<ArgumentValue> Collection { get; set; }

        public ArgumentValue()
            : this("Microsoft.Quantum.Qir.Serialization.ArgumentValue", "ArgumentValue")
        { }

        protected ArgumentValue(string fullName, string name)
        {
            Type = DataType.BytePointer;
        }
    }

    [global::Bond.Schema]
    [System.CodeDom.Compiler.GeneratedCode("gbc", "0.12.1.0")]
    public partial class EntryPointOperation
    {
        [global::Bond.Id(5)]
        public string Name { get; set; }

        [global::Bond.Id(10)]
        public List<Parameter> Parameters { get; set; }

        public EntryPointOperation()
            : this("Microsoft.Quantum.Qir.Serialization.EntryPointOperation", "EntryPointOperation")
        { }

        protected EntryPointOperation(string fullName, string name)
        {
            Name = "";
            Parameters = new List<Parameter>();
        }
    }

    [global::Bond.Schema]
    [System.CodeDom.Compiler.GeneratedCode("gbc", "0.12.1.0")]
    public partial class ExecutionInformation
    {
        [global::Bond.Id(5)]
        public EntryPointOperation EntryPoint { get; set; }

        [global::Bond.Id(10)]
        public Dictionary<string, ArgumentValue> ArgumentValues { get; set; }

        public ExecutionInformation()
            : this("Microsoft.Quantum.Qir.Serialization.ExecutionInformation", "ExecutionInformation")
        { }

        protected ExecutionInformation(string fullName, string name)
        {
            EntryPoint = new EntryPointOperation();
            ArgumentValues = new Dictionary<string, ArgumentValue>();
        }
    }

    [global::Bond.Schema]
    [System.CodeDom.Compiler.GeneratedCode("gbc", "0.12.1.0")]
    public partial class QirExecutionWrapper
    {
        [global::Bond.Id(5)]
        public List<ExecutionInformation> Executions { get; set; }

        [global::Bond.Id(10)]
        public System.ArraySegment<byte> QirBytecode { get; set; }

        public QirExecutionWrapper()
            : this("Microsoft.Quantum.Qir.Serialization.QirExecutionWrapper", "QirExecutionWrapper")
        { }

        protected QirExecutionWrapper(string fullName, string name)
        {
            Executions = new List<ExecutionInformation>();
            QirBytecode = new System.ArraySegment<byte>();
        }
    }
} // Microsoft.Quantum.Qir.Serialization
