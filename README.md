# Edge Vector Index

A fast efficient in-memory vector index designed for use on edge devices.

[BENCHMARKS]

Local is a C# implementation using very similar code. Interop is using the Rust library. It shows that even with a slight FFI overhead the interop to the Rust library performs approximately 7x faster.

| Method                             | Mean       | Error    | StdDev   |
|----------------------------------- |-----------:|---------:|---------:|
| FindClosestMatch_Benchmark_Interop |   594.7 us |  7.16 us |  6.34 us |
| FindClosestMatch_Benchmark_Local   | 4,331.7 us | 12.97 us | 12.13 us |


Dart produces similar benchmarks

Non AOT
Benchmark_Interop(RunTime): 598.3645 us.
Benchmark_Local(RunTime): 5733.497252747253 us.

AOT Times
Benchmark_Interop(RunTime): 589.77 us.
Benchmark_Local(RunTime): 4741.712359550562 us.


Python had the worst runtime for local but similar for interop

Find Closest Match Interop: Mean +- std dev: 612 us +- 12 us
Find Closest Match Local: Mean +- std dev: 26.9 ms +- 0.7 ms


[WARNING] : 
- This library is still under development.
