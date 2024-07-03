import 'package:benchmark_harness/benchmark_harness.dart';
import 'edge_vector_store_local.dart';

class BenchmarkLocal extends BenchmarkBase {
  final EdgeVectorIndexLocal builtIndex;

  final List<double> comparisonVectors;

  BenchmarkLocal(this.builtIndex, this.comparisonVectors) : super('Benchmark_Local');

  @override
  void setup() {
    // No additional setup needed in this case
  }

  @override
  void run() {
    builtIndex.findClosestMatch(comparisonVectors);
  }

  @override
  void exercise() => run();

  @override
  void teardown() {
    // No additional teardown needed in this case
  }
}
