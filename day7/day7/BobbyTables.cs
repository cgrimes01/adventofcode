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

            using(StreamReader file = new StreamReader(filePointer))
            {
                while ((line = file.ReadLine()) != null)
                {
                    computationList.Add(new Computation(line));
                }
            } 
        }

        public UInt16 CalculateValue(string value) {
            UInt16 number;
            if (UInt16.TryParse(value, out number))
            {
                return number;
            }

            UInt16 result;

            Computation computation = computationList.Find(x => x.OutputWire == value.Trim());
            string[] calculationParts = computation.Calculation.Split(' ');

            if(calculationParts.Length == 1)
            {
                result = CalculateValue(computation.Calculation);
            }
            else if (calculationParts[0] == "NOT")
            {
                result = (UInt16)(~CalculateValue(calculationParts[1]));
            }
            else if (calculationParts[1] == "AND")
            {
                result = (UInt16)(CalculateValue(calculationParts[0]) & CalculateValue(calculationParts[2]));
            }
            else if(calculationParts[1] == "OR") {
                result = (UInt16)(CalculateValue(calculationParts[0]) | CalculateValue(calculationParts[2]));
            }
            else if (calculationParts[1] == "LSHIFT")
            {
                result = (UInt16)(CalculateValue(calculationParts[0]) << Int16.Parse(calculationParts[2]));
            }
            else if (calculationParts[1] == "RSHIFT")
            {
                result = (UInt16)(CalculateValue(calculationParts[0]) >> Int16.Parse(calculationParts[2]));
            }
            else
            {
                throw new InvalidDataException($"The calculation [{computation.Calculation}] is not valid for variable {value}");
            }

            computation.Calculation = result.ToString();
            return result;
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
