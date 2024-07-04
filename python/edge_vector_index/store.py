from __future__ import annotations
import ctypes
import typing

from .interop import (
    init_lib,
    create,
    add_to_index,
    find_closest_match,
    free_resources,
    Slicef32,
    Data,
)

T = typing.TypeVar("T")


class Store:
    """Python representation of the C# Store class."""

    def __init__(self):
        """Initializes the store object."""
        init_lib("./libedge_vector_index.so")
        self.handle = create()

    def __del__(self):
        """Releases resources associated with the store."""
        free_resources(self.handle)

    def add_to_index(self, array: list[float], metadata: str) -> None:
        """
        Adds an array to the in-memory index.

        Args:
            array (list[float]): The float array to add.
            metadata (str): The metadata associated with the array (converted to int).
        """
        FloatArray = ctypes.c_float * len(array)
        array_handle = FloatArray(*array)
        data = Data(Slicef32(data=array_handle, len=len(array)), int(metadata))
        add_to_index(self.handle, data)
        del array_handle

    def find_closest_match(self, array: list[float]) -> int:
        """
        Finds the metadata of the closest matching vector.

        Args:
            array (list[float]): The vector to query.

        Returns:
            Union[int, None]: The metadata of the closest match (int), or None if no match found.
        """
        FloatArray = ctypes.c_float * len(array)
        array_handle = FloatArray(*array)
        result = find_closest_match(self.handle, Slicef32(array_handle, len(array)))
        # Handle potential errors from the native library (implementation detail)
        if result == -1:  # Example error handling, adjust based on your library
            return 0
        return result
