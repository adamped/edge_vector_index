import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:main/main.dart';

class BenchmarkInterop extends BenchmarkBase {
 
  final Store builtIndex;

  final List<double> comparisonVectors;

  BenchmarkInterop(this.builtIndex, this.comparisonVectors) : super('Benchmark_Interop');

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
