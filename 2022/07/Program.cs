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
            string[] input = loadInput();
            Console.WriteLine("07");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static string[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static AOCDirectory computeTree(string[] input)
        {
            AOCDirectory homeDir = new AOCDirectory("/");
            Stack<AOCDirectory> dirStack = new Stack<AOCDirectory>();
            AOCDirectory currentDir = homeDir;
            dirStack.Push(currentDir);
            for (int i = 0; i < input.Length; i++)
            {
                string line = input[i];
                string[] cmd = line.Split(" ");
                if (cmd[1] == "cd")
                {
                    if (cmd[2] != "/" && cmd[2] != "..")
                    {
                        currentDir = currentDir.findDir(cmd[2]);
                        dirStack.Push(currentDir);
                    }
                    else if (cmd[2] == "..")
                    {
                        dirStack.Pop();
                        currentDir = dirStack.Peek();
                    }
                }
                else
                {
                    if (line.StartsWith("$"))
                    {
                        string[] output = getOutputByIndex(i, input);
                        i += output.Length;

                        foreach (var outLine in output)
                        {
                            if (outLine.StartsWith("dir"))
                            {
                                string name = outLine.Split(" ")[1];
                                AOCDirectory newDir = new AOCDirectory(name);
                                currentDir.AddDir(newDir);
                            }
                            else
                            {
                                string[] s = outLine.Split(" ");
                                AOCFile file = new AOCFile(s[1], Int32.Parse(s[0]));
                                currentDir.AddFile(file);
                            }
                        }
                    }
                }
            }
            return homeDir;
        }

        static int part1(string[] input)
        {
            AOCDirectory homeDir = computeTree(input);
            int sum = 0;
            foreach (var dir in homeDir.findAllSubDirs())
            {
                int tempSum = dir.findTotalSize();
                if (tempSum <= 100000)
                {
                    sum += tempSum;
                }
            }
            return sum;
        }
        static int part2(string[] input)
        {
            int availableSize = 70000000;
            int neededSpace = 30000000;
            AOCDirectory homeDir = computeTree(input);
            int totalSize = homeDir.findTotalSize();
            neededSpace -= availableSize - totalSize;
            List<AOCDirectory> dirs = homeDir.findAllSubDirs();
            dirs.Add(homeDir);

            for (int i = dirs.Count - 1; i >= 0; i--)
            {
                AOCDirectory d = dirs[i];
                if(d.findTotalSize() < neededSpace){
                    dirs.Remove(d);
                }
            }

            return dirs.Min(x => x.findTotalSize());
        }

        static string[] getOutputByIndex(int index, string[] lines)
        {
            List<string> output = new List<string>();
            for (int i = index + 1; i < lines.Length && !lines[i].StartsWith("$"); i++)
            {
                output.Add(lines[i]);
            }
            return output.ToArray();
        }
    }
}