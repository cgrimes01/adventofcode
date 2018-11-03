using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace day7
{
    public class BobbyTables
    {
        public List<Computation> computationList = new List<Computation>();

        public BobbyTables(string filePointer)
        {
            string line;

            StreamReader file = new StreamReader(filePointer);
            while ((line = file.ReadLine()) != null)
            {
                computationList.Add(new Computation(line));
            }
        }

        public UInt16 CalculateValue(string value) {
            UInt16 number;
            if (UInt16.TryParse(value, out number))
            {
                return number;
            }

            string calculation = computationList.Find(x => x.OutputWire == value.Trim()).Calculation;
            string[] split;
            string operation = "";

            if (calculation.IndexOf("AND") != -1)
            {
                operation = "AND";
                split = calculation.Split(new[] { operation }, StringSplitOptions.None);
                return (UInt16)(CalculateValue(split[0]) & CalculateValue(split[1]));
            }
            else if(calculation.IndexOf("OR") != -1) {
                operation = "OR";
                split = calculation.Split(new[] { operation }, StringSplitOptions.None);
                return (UInt16)(CalculateValue(split[0]) | CalculateValue(split[1]));
            }
            else if (calculation.IndexOf("NOT") != -1)
            {
                operation = "NOT";
                split = calculation.Split(new[] { operation }, StringSplitOptions.None);
                return (UInt16)(~ CalculateValue(split[1]));
            }
            else if (calculation.IndexOf("LSHIFT") != -1)
            {
                operation = "LSHIFT";
                split = calculation.Split(new[] { operation }, StringSplitOptions.None);
                return (UInt16)(CalculateValue(split[0]) << Int16.Parse(split[1]));
            }
            else if (calculation.IndexOf("RSHIFT") != -1)
            {
                operation = "RSHIFT";
                split = calculation.Split(new[] { operation }, StringSplitOptions.None);
                return (UInt16)(CalculateValue(split[0]) >> Int16.Parse(split[1]));
            }

            if(operation == "")
            {
                CalculateValue(calculation);
            }

            return (UInt16)0;
        }
}

    public class Computation
    {
        public string OutputWire { get; set; }
        public string Calculation { get; set; }

        public Computation(string computationText)
        {
            string[] split = computationText.Split(new[] { " -> " }, StringSplitOptions.None);
            OutputWire = split[1];
            Calculation = split[0];
        }
    }

}
