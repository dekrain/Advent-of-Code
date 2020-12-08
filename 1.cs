using System.IO;
using System.Collections.Generic;

static class Program {
    static void Main() {
        Task1();
    }

    static void Task1() {
        System.Console.WriteLine("TASK 1");
        List<int> numbers = new List<int>();
        using (var file = File.OpenText("./1.input")) {
            string line;
            while ((line = file.ReadLine()) != null) {
                numbers.Add(int.Parse(line));
            }
        }
        numbers.Sort();
        foreach (int v in numbers) {
            int id = numbers.BinarySearch(2020 - v);
            if (id >= 0) {
                int w = numbers[id];
                System.Console.WriteLine("Found: {0} and {1}", v, w);
                System.Console.WriteLine("Product is: {0}", v * w);
                break;
            }
        }
    }

    static void Task2() {
        System.Console.WriteLine("TASK 2");
        List<int> numbers = new List<int>();
        using (var file = File.OpenText("./1.input")) {
            string line;
            while ((line = file.ReadLine()) != null) {
                numbers.Add(int.Parse(line));
            }
        }
        numbers.Sort();
        int sz = numbers.Count;
        for (int i = 0; i < sz; ++i) {
            int a = numbers[i];
            if (a > 2020) {
                // Should not happen
                System.Console.WriteLine("WARNING: No answer found");
                break;
            }
            for (int j = 0; j < sz; ++j) {
                int b = numbers[j];
                if (a + b > 2020)
                    break;
                int idx = numbers.BinarySearch(2020 - a - b);
                if (idx >= 0) {
                    int c = numbers[idx];
                    System.Console.WriteLine("Found: {0}, {1}, and {2}", a, b, c);
                    System.Console.WriteLine("Product is: {0}", a * b * c);
                    goto End;
                }
            }
        }
        End:
        ;
    }
}
