namespace Main;

public class Index(float[] vectors, string metadata)
{
    readonly float[] vectors = vectors;
    readonly string metadata = metadata;


    public float[] Vectors { get => vectors; }
    public string Metadata { get => metadata; }
}

public class EdgeVectorIndexLocal
{
    public List<Index> Index { get; private set; }

    public EdgeVectorIndexLocal()
    {
        Index = [];
    }

    public void AddToIndex(List<Index> initialData)
    {
        Index.AddRange(initialData);
    }

    public Index? FindClosestMatch(float[] vector)
    {
        float cosine = 0.0f;
        Index? indexRef = null;

        foreach (var item in Index)
        {
            var similarity = CosineSimilarity(vector, item.Vectors);

            if (similarity >= cosine)
            {
                cosine = similarity;
                indexRef = item;
            }
        }

        return indexRef;
    }

    float CosineSimilarity(float[] vector1, float[] vector2)
    {
        if (vector1.Length != vector2.Length)
        {
            throw new ArgumentException("Vectors must have the same length");
        }

        float dotProductValue = DotProduct(vector1, vector2);
        float sumSq1 = DotProduct(vector1, vector1);
        float sumSq2 = DotProduct(vector2, vector2);

        float magnitude = (float)Math.Sqrt(sumSq1) * (float)Math.Sqrt(sumSq2);

        if (magnitude == 0.0f)
        {
            // With 0 magnitude the relationship is undefined
            return float.NaN;
        }

        return dotProductValue / magnitude;
    }

    static float DotProduct(float[] a, float[] b)
    {
        return Enumerable.Zip(a, b, (x, y) => x * y).Sum();
    }
}
