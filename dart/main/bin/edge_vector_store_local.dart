import 'benchmark.dart';
import 'dart:math';
import 'dart:core';

class EdgeVectorIndexLocal {
  final List<Index> index = [];

  EdgeVectorIndexLocal();

  void addToIndex(List<Index> initialData) {
    index.addAll(initialData);
  }

  Index? findClosestMatch(List<double> vector) {
    double cosine = 0.0;
    Index? closestMatch;

    for (final item in index) {
      final similarity = cosineSimilarity(vector, item.vectors);
      if (similarity >= cosine) {
        cosine = similarity;
        closestMatch = item;
      }
    }

    return closestMatch;
  }

  static double dotProductCalc(List<double> a, List<double> b) {
    return List.generate(a.length, (i) => a[i] * b[i])
        .fold(0.0, (sum, element) => sum + element);
  }

  double cosineSimilarity(List<double> vector1, List<double> vector2) {
    if (vector1.length != vector2.length) {
      throw ArgumentError("Vectors must have the same length");
    }

    final dotProduct = dotProductCalc(vector1, vector2);
    final sumSq1 = dotProductCalc(vector1, vector1);
    final sumSq2 = dotProductCalc(vector2, vector2);

    final magnitude = sqrt(sumSq1) * sqrt(sumSq2);

    if (magnitude == 0.0) {
      // With 0 magnitude the relationship is undefined
      return double.nan;
    }

    return dotProduct / magnitude;
  }
}
