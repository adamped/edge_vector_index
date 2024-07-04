import math
from typing import List, Optional

class Index:
  def __init__(self, vectors: List[float], metadata: str):
    self._vectors = vectors
    self._metadata = metadata

  @property
  def vectors(self) -> List[float]:
    return self._vectors

  @property
  def metadata(self) -> str:
    return self._metadata

class EdgeVectorIndexLocal:
  def __init__(self):
    self.index: List[Index] = []

  def add_to_index(self, initial_data: List[Index]) -> None:
    self.index.extend(initial_data)

  def find_closest_match(self, vector: List[float]) -> Optional[Index]:
    closest_cosine = 0.0
    closest_index: Optional[Index] = None

    for item in self.index:
      similarity = self.cosine_similarity(vector, item.vectors)
      if similarity >= closest_cosine:
        closest_cosine = similarity
        closest_index = item

    return closest_index

  def cosine_similarity(self, vector1: List[float], vector2: List[float]) -> float:
    if len(vector1) != len(vector2):
      raise ValueError("Vectors must have the same length")

    dot_product = sum(a * b for a, b in zip(vector1, vector2))
    sum_sq1 = sum(v * v for v in vector1)
    sum_sq2 = sum(v * v for v in vector2)

    magnitude = math.sqrt(sum_sq1) * math.sqrt(sum_sq2)

    if magnitude == 0.0:
      # Handle undefined case with 0 magnitude
      return float('nan')

    return dot_product / magnitude

  @staticmethod
  def dot_product(a: List[float], b: List[float]) -> float:
    return sum(a * b for a, b in zip(a, b))
