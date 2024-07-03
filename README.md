# Edge Vector Index

A fast efficient in-memory vector index designed for use on edge devices.

[BENCHMARKS]

Local is a C# implementation using very similar code. Interop is using the Rust library. It shows that even with a slight FFI overhead the interop to the Rust library performs approximately 7x faster.

| Method                             | Mean       | Error    | StdDev   |
|----------------------------------- |-----------:|---------:|---------:|
| FindClosestMatch_Benchmark_Interop |   594.7 us |  7.16 us |  6.34 us |
| FindClosestMatch_Benchmark_Local   | 4,331.7 us | 12.97 us | 12.13 us |


[WARNING] : 
- This library is still under development.
