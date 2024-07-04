import sys
sys.path.append( '.' )

from edge_vector_index.store import Store

# Create the store object
store = Store()

# Add some data to the index
store.add_to_index([0.5, 0.1, 0.6], "1")
store.add_to_index([0.1, 0.4, 0.2], "2")

# Find the closest match for a query vector
closest_match = store.find_closest_match([0.1, 0.4, 0.2])

# Print the result (assuming closest_match is an int)
if closest_match is not None:
  print(f"Index Found: {closest_match}")
else:
  print("No match found")

print("Finished")
