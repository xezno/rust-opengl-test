using System;

namespace Test {
    public static class Program {
        public static int Hello(IntPtr args, int sizeBytes) {
            Console.WriteLine("Hello from C#!");
            return 42;
        }

        public void Update() {

        }
    }
}