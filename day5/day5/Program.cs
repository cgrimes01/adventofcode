using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace day5
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] lines = File.ReadAllLines(@"D:\AdventOfCode\adventofcode2015\day5\day5\day5input.txt");
            int niceStringsMethod1 = 0;
            int niceStringsMethod2 = 0;
            foreach (string line in lines)
            {
                if(NaughtyOrNice.IsNiceMethdo1(line)) niceStringsMethod1++;
                if (NaughtyOrNice.IsNiceMethdo2(line)) niceStringsMethod2++;
            }
            Console.WriteLine("Method 1: " + niceStringsMethod1);
            Console.WriteLine("Method 2: " + niceStringsMethod2);
            Console.ReadKey();
        }
    }

    public class NaughtyOrNice
    {
        public static bool IsNiceMethdo1(string input)
        {
            string nicePattern = @"(?=\w*(\w)\1{1,}\w*)(\w*[aeiou]\w*){3,}";
            string bannedPattern = @"^((?!ab)(?!cd)(?!pq)(?!xy).)*$";

            return Regex.IsMatch(input, nicePattern) && Regex.IsMatch(input, bannedPattern);
        }

        public static bool IsNiceMethdo2(string input)
        {
            string nicePattern = @"(?=\w*(\w)\1{1,}\w*)(\w*[aeiou]\w*){3,}";
            string bannedPattern = @"^((?!ab)(?!cd)(?!pq)(?!xy).)*$";

            return Regex.IsMatch(input, nicePattern) && Regex.IsMatch(input, bannedPattern);
        }

        public static bool DoesNotContainBannedCombinations(string input)
        {
            string bannedPattern = @"^((?!ab)(?!cd)(?!pq)(?!xy).)*$";

            return Regex.IsMatch(input, bannedPattern);
        }

        public static bool Contains3OrMoreVowels(string input)
        {
            string vowelsPattern = @"(\w*[aeiou]\w*){3,}";

            return Regex.IsMatch(input, vowelsPattern);
        }

        public static bool ContainsADoubleLetter(string input)
        {
            string doublePattern = @"\w*(\w)\1{1,}\w*";

            return Regex.IsMatch(input, doublePattern);
        }
    }
}
