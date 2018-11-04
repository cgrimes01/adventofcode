using NUnit.Framework;
using System;

namespace day7.tests
{
    [TestFixture]
    public class Part1Tests
    {
        private BobbyTables _bobbyTables;

        [SetUp]
        public void SetUpInit()
        {
            _bobbyTables = new BobbyTables(@"D:\AdventOfCode\adventofcode2015\day7\day7inputtest.txt");
        }

        [Test]
        public void TestDOutput()
        {
            Assert.That(72, Is.EqualTo(_bobbyTables.CalculateValue("d")));
        }

        [Test]
        public void TestEOutput()
        {
            Assert.That(507, Is.EqualTo(_bobbyTables.CalculateValue("e")));
        }

        [Test]
        public void TestFOutput()
        {
            Assert.That(492, Is.EqualTo(_bobbyTables.CalculateValue("f")));
        }

        [Test]
        public void TestGOutput()
        {
            Assert.That(114, Is.EqualTo(_bobbyTables.CalculateValue("g")));
        }

        [Test]
        public void TestHOutput()
        {
            Assert.That(65412, Is.EqualTo(_bobbyTables.CalculateValue("h")));
        }

        [Test]
        public void TestIOutput()
        {
            Assert.That(65079, Is.EqualTo(_bobbyTables.CalculateValue("i")));
        }

        [Test]
        public void TestXOutput()
        {
            Assert.That(123, Is.EqualTo(_bobbyTables.CalculateValue("x")));
        }

        [Test]
        public void TestYOutput()
        {
            Assert.That(456, Is.EqualTo(_bobbyTables.CalculateValue("y")));
        }
    }
}
