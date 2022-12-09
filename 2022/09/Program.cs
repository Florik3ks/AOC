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
            Console.WriteLine("09");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            List<Vector2> visited = new List<Vector2>() { new Vector2() };
            Vector2 h = new Vector2();
            Vector2 t = new Vector2();
            foreach (var line in input)
            {
                string direction = line.Split(" ")[0];
                int count = Int32.Parse(line.Split(" ")[1]);
                for (int i = 0; i < count; i++)
                {
                    int x = 0;
                    int y = 0;
                    switch (direction)
                    {
                        case "U":
                            y += 1;
                            break;
                        case "D":
                            y -= 1;
                            break;
                        case "L":
                            x -= 1;
                            break;
                        case "R":
                            x += 1;
                            break;
                        default:
                            break;
                    }

                    int oldHx = h.x;
                    int oldHy = h.y;

                    h.x += x;
                    h.y += y;
                    if ((Math.Abs(h.x - t.x) > 1 || Math.Abs(h.y - t.y) > 1) && !(Math.Abs(h.x - t.x) == 1 && (Math.Abs(h.y - t.y) == 1)))
                    {
                        t.x = oldHx;
                        t.y = oldHy;
                        if (!visited.Contains(new Vector2(t.x, t.y)))
                        {
                            visited.Add(new Vector2(t.x, t.y));
                        }
                    }
                }
            }
            return visited.Count.ToString();
        }

        static String part2(String[] input)
        {
            int parts = 10;
            Vector2[] tails = new Vector2[parts];
            for (int i = 0; i < parts; i++)
            {
                tails[i] = new Vector2();
            }
            List<Vector2> visited = new List<Vector2>() { new Vector2() };
            Vector2 h = new Vector2();
            Vector2 t = new Vector2();
            foreach (var line in input)
            {
                string direction = line.Split(" ")[0];
                int count = Int32.Parse(line.Split(" ")[1]);
                for (int i = 0; i < count; i++)
                {
                    int x = 0;
                    int y = 0;
                    switch (direction)
                    {
                        case "U":
                            y += 1;
                            break;
                        case "D":
                            y -= 1;
                            break;
                        case "L":
                            x -= 1;
                            break;
                        case "R":
                            x += 1;
                            break;
                        default:
                            break;
                    }

                    tails[0].x += x;
                    tails[0].y += y;

                    for (int j = 1; j < parts; j++)
                    {

                        if (Math.Abs(tails[j - 1].x - tails[j].x) > 1 || Math.Abs(tails[j - 1].y - tails[j].y) > 1)
                        {
                            tails[j].x += Math.Min(Math.Max(-1, tails[j - 1].x - tails[j].x), 1);
                            tails[j].y += Math.Min(Math.Max(-1, tails[j - 1].y - tails[j].y), 1);
                        }
                    }

                    if (!visited.Contains(new Vector2(tails[parts - 1].x, tails[parts - 1].y)))
                    {
                        visited.Add(new Vector2(tails[parts - 1].x, tails[parts - 1].y));
                    }
                }
            }
            return visited.Count.ToString();
        }

    }
}