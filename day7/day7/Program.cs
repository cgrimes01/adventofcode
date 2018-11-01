using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace day7
{
    class Program
    {
        static void Main(string[] args)
        {
            BobbyTables program = new BobbyTables(@"D:\AdventOfCode\adventofcode2015\day7\day7input.txt");
            Console.WriteLine(program.CalculateValue("a"));
            Console.ReadLine();
        }
    }
}
