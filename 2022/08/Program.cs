using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC22
{
    internal class Program
    {
        static List<(int, int)> directions = new List<(int, int)>(){
                { (-1, 0) },  { (1, 0) } , { (0, -1) }, { (0, 1) }
            };
        static void Main(string[] args)
        {
            String[] input = loadInput();
            Console.WriteLine("08");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            int[,] trees = new int[input[0].Length, input.Length];

            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input.Length; x++)
                {
                    trees[x, y] = input[y][x] - '0';
                }
            }
            int count = 0;
            for (int x = 0; x < trees.GetLength(0); x++)
            {
                for (int y = 0; y < trees.GetLength(1); y++)
                {
                    count += isVisible(x, y, trees) ? 1 : 0;
                }
            }
            return count.ToString();
        }
        static bool isVisible(int x, int y, int[,] grid)
        {
            int gridSize = grid.GetLength(0);
            foreach (var (dx, dy) in directions)
            {
                int newX = x + dx;
                int newY = y + dy;

                while (newX >= 0 && newX < gridSize && newY >= 0 && newY < gridSize && grid[newX, newY] < grid[x, y])
                {
                    newX += dx;
                    newY += dy;
                }

                if (!(newX >= 0 && newX < gridSize && newY >= 0 && newY < gridSize))
                {
                    return true;
                }
            }
            return false;
        }

        static String part2(String[] input)
        {
            int[,] trees = new int[input[0].Length, input.Length];

            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input.Length; x++)
                {
                    trees[x, y] = input[y][x] - '0';
                }
            }
            int maxCount = 0;
            for (int x = 0; x < trees.GetLength(0); x++)
            {
                for (int y = 0; y < trees.GetLength(1); y++)
                {
                    maxCount = Math.Max(maxCount, getScenicView(x, y, trees));
                }
            }
            return maxCount.ToString();
        }
        static int getScenicView(int x, int y, int[,] grid)
        {
            int gridSize = grid.GetLength(0);
            if(x == 0 || y == 0 || x == gridSize - 1 || y == gridSize - 1) return 0;
            int score = 1;
            foreach (var (dx, dy) in directions)
            {
                int tempScore = 1;
                int newX = x + dx;
                int newY = y + dy;

                while (newX > 0 && newX < gridSize - 1 && newY > 0 && newY < gridSize - 1 && grid[newX, newY] < grid[x, y])
                {
                    tempScore++;
                    newX += dx;
                    newY += dy;
                }

                score *= tempScore;
            }
            return score;
        }
    }
}