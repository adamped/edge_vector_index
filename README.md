# Edge Vector Index

A fast efficient in-memory vector index designed for use on edge devices.

[BENCHMARKS]

Local is a C# implementation using very similar code. Interop is using the Rust library. It shows that even with a slight FFI overhead the interop to the Rust library performs approximately 7x faster.

| Method                             | Mean      | Error    | StdDev   |
|----------------------------------- |----------:|---------:|---------:|
| FindClosestMatch_Benchmark_Interop |  59.25 us | 0.081 us | 0.076 us |
| FindClosestMatch_Benchmark_Local   | 417.88 us | 0.390 us | 0.364 us |


[WARNING] : 
- This library is still under development.
