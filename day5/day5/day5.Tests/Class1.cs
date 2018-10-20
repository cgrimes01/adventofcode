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
        public void IsANiceStringExample1()
        {
            var result = NaughtyOrNice.IsNice("ugknbfddgicrmopn");
            Assert.IsTrue(result);
        }

        [Test]
        public void IsANiceStringExample2()
        {
            var result = NaughtyOrNice.IsNice("aaa");
            Assert.IsTrue(result);
        }

        [Test]
        public void IsANiceStringExample3()
        {
            var result = NaughtyOrNice.IsNice("jchzalrnumimnmhp");
            Assert.IsFalse(result);
        }

        [Test]
        public void IsANiceStringExample4()
        {
            var result = NaughtyOrNice.IsNice("haegwjzuvuyypxyu");
            Assert.IsFalse(result);
        }

        [Test]
        public void IsANiceStringExample5()
        {
            var result = NaughtyOrNice.IsNice("dvszwmarrgswjxmb");
            Assert.IsFalse(result);
        }
    }
}
