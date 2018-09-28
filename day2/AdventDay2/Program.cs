using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventDay2
{
    class Program
    {
        static void Main(string[] args)
        {
            var filePath = Console.ReadLine();
            var fileText = File.ReadAllLines(filePath);

            var presents = 
                fileText
                    .Select(line => 
                        line.Split('x')
                            .Select(item => {
                                int value;
                                bool success = int.TryParse(item, out value);
                                return success ? value : 0;
                            })
                            .OrderBy(item => item)
                            .ToArray()
                    )
                    .Select(line =>
                        new {wrappingPaper = (3 * line[0] * line[1]) + (2 * line[1] * line[2]) + (2 * line[0] * line[2]),
                             ribbon = (line[0] * 2) + (line[1] * 2) + (line[0] * line[1] * line[2])})
                    
                    ;

            Console.WriteLine("Wrapping Paper:" + presents.Select(present => present.wrappingPaper).Sum());
            Console.WriteLine("Ribbon:" + presents.Select(present => present.ribbon).Sum());
            Console.ReadKey();
        }
    }
}
