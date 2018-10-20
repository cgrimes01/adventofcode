using System;
using System.Collections.Generic;
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
            Console.WriteLine("Hello World!");
            Console.ReadKey();
        }
    }

    public class NaughtyOrNice
    {
        public static bool IsNice(string input)
        {
            string nicePattern = "(?=\\w*(\\w)\\1{1,}\\w*)(\\w*[aeiou]\\w*){3,}";
            string bannedPattern = "^((?!ab)(?!cd)(?!pq)(?!xy).)*$";

            return Regex.IsMatch(input, nicePattern) && Regex.IsMatch(input, bannedPattern);
        }

        public static bool DoesNotContainBannedCombinations(string input)
        {
            string bannedPattern = "^((?!ab)(?!cd)(?!pq)(?!xy).)*$";

            return Regex.IsMatch(input, bannedPattern);
        }

        public static bool Contains3OrMoreVowels(string input)
        {
            string vowelsPattern = "(\\w*[aeiou]\\w*){3,}";

            return Regex.IsMatch(input, vowelsPattern);
        }

        public static bool ContainsADoubleLetter(string input)
        {
            string doublePattern = "\\w*(\\w)\\1{1,}\\w*";

            return Regex.IsMatch(input, doublePattern);
        }
    }
}