using BenchmarkDotNet.Running;
using Benchmarks;
using EdgeVectorIndex;

BenchmarkRunner.Run<Benchmark>();

var store = new Store();

store.AddToIndex([0.5f, 0.1f, 0.6f], "1");
store.AddToIndex([0.1f, 0.4f, 0.2f], "2");

Console.WriteLine($"Index Found: {store.FindClosestMatch([0.4f, 0.1f, 0.7f])}");

Console.WriteLine("Finished");