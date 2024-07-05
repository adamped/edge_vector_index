# Edge Vector Index

A fast efficient in-memory vector index designed for use on edge devices.

## WARNING 
- This library is experimental and uses simple cosine similarity for comparison

## BENCHMARKS

### C# 
Local is a C# implementation using very similar code. Interop is using the Rust library. It shows that even with a slight FFI overhead the interop to the Rust library performs approximately 7x faster.

| Method                             | Mean       | Error    | StdDev   |
|----------------------------------- |-----------:|---------:|---------:|
| FindClosestMatch_Benchmark_Interop |   594.7 us |  7.16 us |  6.34 us |
| FindClosestMatch_Benchmark_Local   | 4,331.7 us | 12.97 us | 12.13 us |

### Dart
Dart produces similar benchmarks

Non AOT

| Method                     | Mean         |
|--------------------------- |-------------:|
| Benchmark_Interop(RunTime) |  598.3645 us |
| Benchmark_Local(RunTime)   | 5733.4972 us | 


AOT

| Method                     | Mean         |
|--------------------------- |-------------:|
| Benchmark_Interop(RunTime) |  589.7700 us |
| Benchmark_Local(RunTime)   | 4741.7123 us | 


### Python
Python had the worst runtime for local but similar for interop

| Method                     | Mean       | StdDev   |
|--------------------------- |-----------:|---------:|
| Find Closest Match Interop |     612 us |    12 us |
| Find Closest Match Local   |    26.9 ms |   0.7 us |
