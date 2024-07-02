using System.Runtime.InteropServices;

namespace EdgeVectorIndex
{
    public sealed class Store
    {
        readonly IntPtr handle;

        public Store()
        {
            handle = Interop.create();
        }

        ~Store()
        {
            Interop.free_resources(handle);
        }

        /// <summary>
        /// Adds array to in-memory index
        /// </summary>
        /// <param name="array">Float array for vector index</param>
        /// <param name="metadata">Will be converted to UTF8</param>
        public void AddToIndex(float[] array, string metadata)
        {
            var arrayHandle = GCHandle.Alloc(array, GCHandleType.Pinned);
            var data = new Data() { data = new Slicef32(arrayHandle, (ulong)array.Length), id = int.Parse(metadata) };
            Interop.add_to_index(handle, data);
            arrayHandle.Free();
        }

        /// <summary>
        /// Will return the metadata of the closest matching vector
        /// </summary>
        /// <param name="array">Vector</param>
        /// <returns></returns>
        public string FindClosestMatch(float[] array)
        {
            var arrayHandle = GCHandle.Alloc(array, GCHandleType.Pinned);
            var value = Interop.find_closest_match(handle, new Slicef32(arrayHandle, (ulong)array.Length)).ToString();
            arrayHandle.Free();
            return value;
        }
    }
}
