namespace Benchmarks;
using BenchmarkDotNet.Attributes;
using EdgeVectorIndex;
using Main;

public class Benchmark
{
    Store _builtIndexStore = BuildIndex();
    EdgeVectorIndexLocal _builtIndexLocal = BuildIndexLocal();
    float[] _comparisonVectors = BuildComparison().Vectors.ToArray();

    [GlobalSetup]
    public void Setup()
    {

    }

    [Benchmark]
    public string FindClosestMatch_Benchmark_Interop()
    {
        return _builtIndexStore.FindClosestMatch(_comparisonVectors);
    }

    [Benchmark]
    public Index? FindClosestMatch_Benchmark_Local()
    {
        return _builtIndexLocal.FindClosestMatch(_comparisonVectors);
    }

    const int indexLength = 1000;

    static Store BuildIndex()
    {
        var index = new Store();
        var indexes = new List<Index>();

        for (int i = 0; i < indexLength; i++)
        {
            indexes.Add(BuildComparison());
        }

        foreach (var idx in indexes)
        {
            index.AddToIndex(idx.Vectors, idx.Metadata);
        }
        return index;
    }

    static EdgeVectorIndexLocal BuildIndexLocal()
    {
      var index = new EdgeVectorIndexLocal();
        var indexes = new List<Index>();

        for (int i = 0; i < indexLength; i++)
        {
            indexes.Add(BuildComparison());
        }

        index.AddToIndex(indexes);
        return index;
    }

    static Index BuildComparison()
    {
        var numbers = new List<float>();
        for (int i = 0; i < 384; i++)
        {
            numbers.Add(0.1f);
        }

        return new Index([.. numbers], "0");
    }
}
