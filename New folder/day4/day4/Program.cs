using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

namespace day4
{
    class Program
    {
        static void Main(string[] args)
        {
            var input = "ckczppom";
            var answer = 0;
            var matchFound = false;
            var numberOfLeadingZeroes = 6;

            StringBuilder zeroSb = new StringBuilder();
            for(var i = 0; i< numberOfLeadingZeroes; i++)
            {
                zeroSb.Append("0");
            }
            var zeroString = zeroSb.ToString();

            MD5 md5Hash = MD5.Create();

            while (!matchFound)
            {
                answer++;
                if (Md5HashHasLeadingZeroes(md5Hash, input + answer, numberOfLeadingZeroes, zeroString))
                {
                    matchFound = true;
                }
            }

            Console.WriteLine(answer);
            Console.ReadKey();
        }

        static bool Md5HashHasLeadingZeroes(MD5 md5Hash, string input, int leadingZeroDigits, string desiredResult)
        {

            // Convert the input string to a byte array and compute the hash.
            byte[] data = md5Hash.ComputeHash(Encoding.UTF8.GetBytes(input));

            // Create a new Stringbuilder to collect the bytes
            // and create a string.
            StringBuilder sBuilder = new StringBuilder();

            for (int i = 0; i <= leadingZeroDigits/2; i++)
            {
                sBuilder.Append(data[i].ToString("x2"));
            }

            sBuilder.Length = leadingZeroDigits;

            return sBuilder.ToString() == desiredResult;
        }

        static bool Md5HashHas5LeadingZeroes(MD5 md5Hash, string input)
        {

            // Convert the input string to a byte array and compute the hash.
            byte[] data = md5Hash.ComputeHash(Encoding.UTF8.GetBytes(input));

            var first5 = data[0].ToString("x2") + data[1].ToString("x2") + data[2].ToString("x2").Substring(0, 1);

            return first5 == "00000";
        }

    }
}
