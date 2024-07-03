use std::slice;

use interoptopus::{
    ffi_function, ffi_type, function, patterns::slice::FFISlice, Inventory, InventoryBuilder,
};

use crate::{EdgeVectorIndex, Index};

#[ffi_type()]
#[repr(C)]
pub struct Data<'a> {
    pub data: FFISlice<'a, f32>,
    pub id: i32,
}

#[ffi_type(opaque)]
#[repr(C)]
pub struct EdgeVectorIndexHandle {
    instance: *mut EdgeVectorIndex,
}

impl EdgeVectorIndexHandle {
    fn out_of_rust(value: EdgeVectorIndex) -> EdgeVectorIndexHandle {
        EdgeVectorIndexHandle {
            instance: Box::into_raw(Box::new(value)),
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn free_resources(handle: EdgeVectorIndexHandle) {
    // Captures it via Box, then Rust deals with it.
    unsafe {
        let _ = &mut *(Box::from_raw(handle.instance));
    };
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn find_closest_match(
    handle: EdgeVectorIndexHandle,
    vector: FFISlice<'_, f32>,
) -> i32 {
    let store = unsafe { &mut *(handle.instance) };
    let index = store.find_closest_match(slice_to_array(vector));

    if index.is_none() {
        return 0;
    }

    // TODO: Change to string
    index.unwrap().metadata.parse().unwrap()
}

fn slice_to_array(vector: FFISlice<'_, f32>) -> &[f32] {
    let data_len = vector.len();

    unsafe { slice::from_raw_parts(vector.as_ptr(), data_len) }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn create() -> EdgeVectorIndexHandle {
    EdgeVectorIndexHandle::out_of_rust(EdgeVectorIndex::new())
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn add_to_index(handle: EdgeVectorIndexHandle, data: Data) {
    let store = unsafe { &mut *(handle.instance) };

    let vec = slice_to_array(data.data);

    let mut indexes = Vec::new();
    indexes.push(Index::new(vec.to_vec(), data.id.to_string()));

    store.add_to_index(indexes);
}

pub fn create_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(create))
        .register(function!(add_to_index))
        .register(function!(find_closest_match))
        .register(function!(free_resources))
        .inventory()
}
