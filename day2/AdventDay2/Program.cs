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
            var fileStream = new FileStream(filePath, FileMode.Open, FileAccess.Read, FileShare.ReadWrite);
            var file = new StreamReader(fileStream, Encoding.UTF8, true, 128);

            string lineOfText;
            while ((lineOfText = file.ReadLine()) != null)
            {

                var lineNumbers = 
                    lineOfText.Split('x')
                        .Select(item => {
                            int value;
                            bool success = int.TryParse(item, out value);
                            return success ? value : 0;
                        });

                int product = lineNumbers.Aggregate(1, (x, y) => x * y);


                Console.WriteLine("Input: " + lineOfText + "...." + product.ToString());
            }

            Console.ReadKey();
        }
    }
}
