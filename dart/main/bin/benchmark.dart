const indexLength = 1000;

class Index {
  final List<double> vectors;
  final String metadata;

  const Index(this.vectors, this.metadata);

  List<double> get getVectors => vectors;
  String get getMetadata => metadata;

  factory Index.buildComparison() {
    final numbers = List.generate(384, (_) => 0.1);
    return Index(numbers, "0");
  }
}