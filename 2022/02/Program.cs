using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC22
{
    internal class Program
    {
        static void Main(string[] args)
        {
            String[] input = loadInput();
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            int score = 0;
            Dictionary<string, int> scores = new Dictionary<string, int>();
            scores.Add("A", 1);
            scores.Add("B", 2);
            scores.Add("C", 3);

            scores.Add("X", 1);
            scores.Add("Y", 2);
            scores.Add("Z", 3);

            foreach (String line in input)
            {
                var values = line.Split(" ");

                if (scores[values[0]] == scores[values[1]])
                {
                    score += scores[values[1]] + 3;
                }

                else if (values[0] == "A")
                {
                    if (values[1] == "Y")
                    {
                        score += scores[values[1]] + 6;
                    }
                    else if (values[1] == "Z")
                    {
                        score += scores[values[1]];
                    }
                }

                else if (values[0] == "B")
                {
                    if (values[1] == "Z")
                    {
                        score += scores[values[1]] + 6;
                    }
                    else if (values[1] == "X")
                    {
                        score += scores[values[1]];
                    }
                }


                else if (values[0] == "C")
                {
                    if (values[1] == "X")
                    {
                        score += scores[values[1]] + 6;
                    }
                    else if (values[1] == "Y")
                    {
                        score += scores[values[1]];
                    }-
                }
            }
            return score.ToString();
        }

        static String part2(String[] input)
        {
            int score = 0;
            Dictionary<string, int> scores = new Dictionary<string, int>();
            scores.Add("A", 1);
            scores.Add("B", 2);
            scores.Add("C", 3);

            scores.Add("X", 1);
            scores.Add("Y", 2);
            scores.Add("Z", 3);

            foreach (String line in input)
            {
                var values = line.Split(" ");

                if (values[1] == "X")
                {
                    if (values[0] == "A")
                    {
                        score += scores["Z"];
                    }
                    else if (values[0] == "B")
                    {
                        score += scores["X"];
                    }
                    else if (values[0] == "C")
                    {
                        score += scores["Y"];
                    }
                }

                else if (values[1] == "Y")
                {
                    if (values[0] == "A")
                    {
                        score += scores["X"] + 3;
                    }
                    else if (values[0] == "B")
                    {
                        score += scores["Y"] + 3;
                    }
                    else if (values[0] == "C")
                    {
                        score += scores["Z"] + 3;
                    }
                }

                else if (values[1] == "Z")
                {
                    if (values[0] == "A")
                    {
                        score += scores["Y"] + 6;
                    }
                    else if (values[0] == "B")
                    {
                        score += scores["Z"] + 6;
                    }
                    else if (values[0] == "C")
                    {
                        score += scores["X"] + 6;
                    }
                }
            }
            return score.ToString();
        }

    }
}