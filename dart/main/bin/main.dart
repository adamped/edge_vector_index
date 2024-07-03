import 'package:main/main.dart' as lib;
import 'package:main/main.dart';
import 'benchmark_interop.dart';
import 'benchmark_local.dart';
import 'benchmark.dart';
import 'edge_vector_store_local.dart';

void main(List<String> arguments) {
  var index = buildIndex();
  var comparison = Index.buildComparison().vectors;
  var localIndex = buildLocalIndex();

  BenchmarkInterop(index, comparison).report();
  BenchmarkLocal(localIndex, comparison).report();

  var store = lib.Store();

  store.addToIndex([0.1, 0.2], "1");
  store.addToIndex([0.3, 0.4], "2");

  print(store.findClosestMatch([0.3, 0.4]));

  store.dispose();

  print('Finished');
}

Store buildIndex() {
  final store = Store();
  final indexes =
      List<Index>.generate(indexLength, (_) => Index.buildComparison());

  for (final idx in indexes) {
    store.addToIndex(idx.vectors, idx.metadata);
  }

  return store;
}

EdgeVectorIndexLocal buildLocalIndex() {
  final store = EdgeVectorIndexLocal();
  final indexes =
      List<Index>.generate(indexLength, (_) => Index.buildComparison());

  store.addToIndex(indexes);

  return store;
}
