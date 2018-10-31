using NUnit.Framework;

namespace day5.Tests
{
    [TestFixture]
    public class Part1Tests
    {
        [Test]
        public void StringContains3Vowels()
        {
            var result = NaughtyOrNice.Contains3OrMoreVowels("uatoibc");
            Assert.IsTrue(result);
        }
        [Test]
        public void StringNotContains3Vowels()
        {
            var result = NaughtyOrNice.Contains3OrMoreVowels("gabtyursw");
            Assert.IsFalse(result);
        }
        [Test]
        public void StringContainsADoubleLetter()
        {
            var result = NaughtyOrNice.ContainsADoubleLetter("uggabeka");
            Assert.IsTrue(result);
        }
        [Test]
        public void StringNotContainsADoubleLetter()
        {
            var result = NaughtyOrNice.ContainsADoubleLetter("ugabeka");
            Assert.IsFalse(result);
        }

        [Test]
        public void StringContainsBannedPattern()
        {
            var result = NaughtyOrNice.DoesNotContainBannedCombinations("abukeor");
            Assert.IsFalse(result);
        }
        [Test]
        public void StringNotContainsBannedPattern()
        {
            var result = NaughtyOrNice.DoesNotContainBannedCombinations("ugabeka");
            Assert.IsFalse(result);
        }

        [Test]
        public void IsANiceStringExample1()
        {
            var result = NaughtyOrNice.IsNiceMethod1("ugknbfddgicrmopn");
            Assert.IsTrue(result);
        }

        [Test]
        public void IsANiceStringExample2()
        {
            var result = NaughtyOrNice.IsNiceMethod1("aaa");
            Assert.IsTrue(result);
        }

        [Test]
        public void IsANiceStringExample3()
        {
            var result = NaughtyOrNice.IsNiceMethod1("jchzalrnumimnmhp");
            Assert.IsFalse(result);
        }

        [Test]
        public void IsANiceStringExample4()
        {
            var result = NaughtyOrNice.IsNiceMethod1("haegwjzuvuyypxyu");
            Assert.IsFalse(result);
        }

        [Test]
        public void IsANiceStringExample5()
        {
            var result = NaughtyOrNice.IsNiceMethod1("dvszwmarrgswjxmb");
            Assert.IsFalse(result);
        }
    }

    [TestFixture]
    public class Part2Tests
    {
        [Test]
        public void IsANiceStringExample1()
        {
            var result = NaughtyOrNice.IsNiceMethod2("qjhvhtzxzqqjkmpb");
            Assert.IsTrue(result);
        }

        [Test]
        public void IsANiceStringExample2()
        {
            var result = NaughtyOrNice.IsNiceMethod2("xxyxx");
            Assert.IsTrue(result);
        }

        [Test]
        public void IsANiceStringExample3()
        {
            var result = NaughtyOrNice.IsNiceMethod2("uurcxstgmygtbstg");
            Assert.IsFalse(result);
        }

        [Test]
        public void IsANiceStringExample4()
        {
            var result = NaughtyOrNice.IsNiceMethod2("ieodomkazucvgmuy");
            Assert.IsFalse(result);
        }
    }
}
