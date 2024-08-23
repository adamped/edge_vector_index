// Automatically generated by Interoptopus.

#ifndef edge_vector_index
#define edge_vector_index

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>




typedef struct edgevectorindexhandle edgevectorindexhandle;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct slicef32
    {
    ///Pointer to start of immutable data.
    const float* data;
    ///Number of elements.
    uint64_t len;
    } slicef32;

typedef struct data
    {
    slicef32 data;
    int32_t id;
    } data;


edgevectorindexhandle create();

void add_to_index(edgevectorindexhandle handle, data data);

int32_t find_closest_match(edgevectorindexhandle handle, slicef32 vector);

void free_resources(edgevectorindexhandle handle);

float cosine_similarity(slicef32 input, slicef32 output);


#ifdef __cplusplus
}
#endif

#endif /* edge_vector_index */
