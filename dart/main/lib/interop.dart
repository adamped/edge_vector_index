import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'package:ffi/ffi.dart';

typedef EdgeVectorIndexHandle = ffi.Pointer;

final dylib = ffi.DynamicLibrary.open('assets/libedge_vector_index.so');

// Create
typedef CreateFunc = ffi.Pointer Function();
typedef Create = EdgeVectorIndexHandle Function();

final Create create =
    dylib.lookup<ffi.NativeFunction<CreateFunc>>('create').asFunction();

// Add To Index
typedef AddToIndexFunc = ffi.Void Function(
    EdgeVectorIndexHandle handle, Data data);
typedef AddToIndex = void Function(EdgeVectorIndexHandle handle, Data data);

final AddToIndex addToIndex = dylib
    .lookup<ffi.NativeFunction<AddToIndexFunc>>('add_to_index')
    .asFunction();

// Find Closest Match
typedef FindClosestMatchFunc = ffi.Int Function(
    EdgeVectorIndexHandle handle, Slicef32 vector);
typedef FindClosestMatch = int Function(
    EdgeVectorIndexHandle handle, Slicef32 vector);

final FindClosestMatch findClosestMatch = dylib
    .lookup<ffi.NativeFunction<FindClosestMatchFunc>>('find_closest_match')
    .asFunction();

// Free Resources
typedef FreeResourcesFunc = ffi.Void Function(EdgeVectorIndexHandle handle);
typedef FreeResourcesMatch = void Function(EdgeVectorIndexHandle handle);

final FreeResourcesMatch freeResources = dylib
    .lookup<ffi.NativeFunction<FreeResourcesFunc>>('free_resources')
    .asFunction();

/// A pointer to an array of data someone else owns which may not be modified.
final class Slicef32 extends ffi.Struct {
  /// Pointer to start of immutable data.
  external ffi.Pointer<ffi.Float> data;

  /// Number of elements.
  @ffi.Uint64()
  external int len;

  factory Slicef32(List<double> data, int len) {
    return Struct.create()
      ..data = doubleListToArray(data)
      ..len = len;
  }
}

final class Data extends ffi.Struct {
  external Slicef32 data1;

  @ffi.Int32()
  external int id;

  factory Data(Slicef32 data, int id) {
    return Struct.create()
      ..data1 = data
      ..id = id;
  }
}

Pointer<ffi.Float> doubleListToArray(List<double> list) {
  final ptr = malloc.allocate<ffi.Float>(sizeOf<ffi.Float>() * list.length);
  for (var i = 0; i < list.length; i++) {
    ptr[i] = list[i];
  }
  return ptr;
}
