
import 'dart:ffi';

import 'package:main/interop.dart' as interop;

class Store {
  final Pointer<NativeType> handle;

  Store() : handle = interop.create();

  void dispose() {
    interop.freeResources(handle);
  }

  /// Adds array to in-memory index
  ///
  /// Adds a float array for vector index with associated metadata.
  /// Metadata will be converted to an integer.
  void addToIndex(List<double> array, String metadata) {
    final slicef32 = interop.Slicef32(array, array.length);
    final data = interop.Data(slicef32, int.parse(metadata));
    interop.addToIndex(handle, data);
  }

  /// Will return the metadata of the closest matching vector
  ///
  /// Finds the closest matching vector based on the provided array.
  /// Returns the metadata (converted to a string) of the closest match.
  String findClosestMatch(List<double> array) {
    final slicef32 = interop.Slicef32(array, array.length);
    final value = interop.findClosestMatch(handle, slicef32).toString();
    return value;
  }
}


