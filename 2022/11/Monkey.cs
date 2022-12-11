using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC22
{
    public class Monkey
    {
        private long inspectCount = 0;
        public long InspectCount { get => inspectCount; }
        public delegate void ThrowToMonkey(int monkey, long num);
        public delegate long Operator(long num);
        public int Name { get => name; }
        private int name;
        private string operation;
        private List<long> startingNums;
        private int divisibleByX;
        public int DivisibleByX { get => divisibleByX; }
        private int monkeyTrue;
        private int monkeyFalse;
        public Monkey(int name, List<long> startingNums, string operation, int monkeyTrue, int monkeyFalse, int divisibleByX)
        {
            this.name = name;
            this.startingNums = startingNums;
            this.operation = operation;
            this.monkeyTrue = monkeyTrue;
            this.monkeyFalse = monkeyFalse;
            this.divisibleByX = divisibleByX;
        }
        public void AddToList(long item)
        {
            startingNums.Add(item);
        }
        public void TakeTurn(ThrowToMonkey throwTo, Operator op)
        {
            int toRemove = 0;
            if (startingNums.Count == 0)
            {
                return;
            }
            for (int i = 0; i < startingNums.Count; i++)
            {
                startingNums[i] = InspectItem(startingNums[i]);

                startingNums[i] = op(startingNums[i]);

                if (TestItem(startingNums[i]))
                {
                    throwTo(monkeyTrue, startingNums[i]);
                }
                else
                {
                    throwTo(monkeyFalse, startingNums[i]);
                }
                toRemove++;
            }
            startingNums.RemoveRange(0, toRemove);
        }
        private bool TestItem(long item)
        {
            return item % divisibleByX == 0;
        }
        private long InspectItem(long item)
        {
            inspectCount++;
            string numStr = operation.Split(" ").Last();

            long num;
            if (numStr == "old")
            {
                num = item;
            }
            else
            {
                num = long.Parse(numStr);
            }
            switch (operation.Split(" ").ElementAt(operation.Split(" ").Length - 2))
            {
                case "*":
                    item *= num;
                    return item;
                case "+":
                    item += num;
                    return item;
                default:
                    return item;
            }
        }
        public static Monkey CreateMonkeyFromInput(string[] input)
        {
            int name = Int32.Parse(input[0].Replace(":", "").Split(" ")[1]);
            List<long> nums = new List<long>();
            foreach (var item in input[1].Replace("  Starting items: ", "").Split(", "))
            {
                nums.Add(long.Parse(item));
            }
            var temp = input[2].Split(" ");
            string operation = temp.ElementAt(temp.Length - 2) + " " + temp.Last();
            int divisibleByX = int.Parse(input[3].Split(" ").Last());
            int t = Int32.Parse(input[4].Split(" ").Last());
            int f = Int32.Parse(input[5].Split(" ").Last());
            return new Monkey(name, nums, operation, t, f, divisibleByX);
        }
    }
}