from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
EdgeVectorIndexHandle = ctypes.c_void_p;
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.create.argtypes = []
    c_lib.add_to_index.argtypes = [EdgeVectorIndexHandle, Data]
    c_lib.find_closest_match.argtypes = [EdgeVectorIndexHandle, Slicef32]
    c_lib.free_resources.argtypes = [EdgeVectorIndexHandle]

    c_lib.create.restype = EdgeVectorIndexHandle
    c_lib.find_closest_match.restype = ctypes.c_int32



def create():
    return c_lib.create()

def add_to_index(handle: EdgeVectorIndexHandle, data: Data):
    return c_lib.add_to_index(handle, data)

def find_closest_match(handle: EdgeVectorIndexHandle, vector: Slicef32 | ctypes.Array[ctypes.c_float]) -> int:
    if hasattr(vector, "_length_") and getattr(vector, "_type_", "") == ctypes.c_float:
        vector = Slicef32(data=ctypes.cast(vector, ctypes.POINTER(ctypes.c_float)), len=len(vector))

    return c_lib.find_closest_match(handle, vector)

def free_resources(handle: EdgeVectorIndexHandle):
    return c_lib.free_resources(handle)





TRUE = ctypes.c_uint8(1)
FALSE = ctypes.c_uint8(0)


def _errcheck(returned, success):
    """Checks for FFIErrors and converts them to an exception."""
    if returned == success: return
    else: raise Exception(f"Function returned error: {returned}")


class CallbackVars(object):
    """Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks."""
    def __str__(self):
        rval = ""
        for var in  filter(lambda x: "__" not in x, dir(self)):
            rval += f"{var}: {getattr(self, var)}"
        return rval


class _Iter(object):
    """Helper for slice iterators."""
    def __init__(self, target):
        self.i = 0
        self.target = target

    def __iter__(self):
        self.i = 0
        return self

    def __next__(self):
        if self.i >= self.target.len:
            raise StopIteration()
        rval = self.target[self.i]
        self.i += 1
        return rval


class Slicef32(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_float)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> float:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> Slicef32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_float * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_float))
        rval = Slicef32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_float)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_float]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_float]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> float:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> float:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class Data(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", Slicef32),
        ("id", ctypes.c_int32),
    ]

    def __init__(self, data: Slicef32 = None, id: int = None):
        if data is not None:
            self.data = data
        if id is not None:
            self.id = id

    @property
    def data(self) -> Slicef32:
        return ctypes.Structure.__get__(self, "data")

    @data.setter
    def data(self, value: Slicef32):
        return ctypes.Structure.__set__(self, "data", value)

    @property
    def id(self) -> int:
        return ctypes.Structure.__get__(self, "id")

    @id.setter
    def id(self, value: int):
        return ctypes.Structure.__set__(self, "id", value)




class callbacks:
    """Helpers to define callbacks."""


