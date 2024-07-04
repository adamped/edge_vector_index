from typing import List
from local_store import EdgeVectorIndexLocal, Index
import pyperf
import sys
sys.path.append( '.' )

from edge_vector_index.store import Store

index_count = 1000

def build_comparison() -> Index:
  numbers = [1 for _ in range(384)] 
  return Index(numbers, "0")

def build_index(index_length: int) -> Store:
  store = Store()
  indexes: List[Index] = []

  # Create the desired number of Index objects with comparison data
  for _ in range(index_length):
    indexes.append(build_comparison())

  # Add each Index object to the store using vector and metadata
  for idx in indexes:
    store.add_to_index(idx.vectors, idx.metadata)

  return store

def build_local_index(index_length: int) -> EdgeVectorIndexLocal:
    store = EdgeVectorIndexLocal()
    indexes: List[Index] = []

    # Create the desired number of Index objects with comparison data
    for _ in range(index_length):
        indexes.append(build_comparison())
    
    store.add_to_index(indexes)

    return store

# Create the store object
store = build_index(index_count)

local_store = build_local_index(index_count)

comparison_vectors = build_comparison()

def benchmark_interop():
    store.find_closest_match(comparison_vectors.vectors)

def benchmark_local():
    local_store.find_closest_match(comparison_vectors.vectors)

runner = pyperf.Runner()
runner.timeit(name="Find Closest Match Interop",
              stmt="benchmark_interop()",
              setup="from __main__ import benchmark_interop")


runner.timeit(name="Find Closest Match Local",
              stmt="benchmark_local()",
              setup="from __main__ import benchmark_local")


