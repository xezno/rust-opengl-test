using System;
using System.Runtime.InteropServices;

namespace Test
{
    [StructLayout(LayoutKind.Sequential)]
    struct Test
    {
        public IntPtr name;
        public int age;
        public IntPtr callback;
    }

    public delegate void Callback([MarshalAs(UnmanagedType.LPStr)] string str);

    public static class Program
    {
        public static int Hello(IntPtr args, int sizeBytes)
        {
            var test = Marshal.PtrToStructure<Test>(args);

            var nameStr = Marshal.PtrToStringUTF8(test.name);

            Console.WriteLine($"Name: {nameStr}");
            Console.WriteLine($"Age: {test.age}");

            var callbackTestFn = Marshal.GetDelegateForFunctionPointer<Callback>(test.callback);
            callbackTestFn.Invoke("8======D hehe");

            return test.age;
        }
    }
}