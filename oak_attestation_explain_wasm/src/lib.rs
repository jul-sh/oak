//
// Copyright 2024 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// TODO: b/351012006 - Load the attestation library.
// Right now this is just a placeholder for Wasm logic without generated
// bindings. From: https://surma.dev/things/rust-to-webassembly/

#[no_mangle]
// Safety: arguments must be a valid pointer that is safe to deference.
pub unsafe extern "C" fn process_bytes_with_random(ptr: *const u8, len: usize) -> *mut u8 {
    let input = std::slice::from_raw_parts(ptr, len);

    // Create a new vector with the same length as the input
    let mut output = Vec::with_capacity(input.len());

    // Get some random bytes
    let mut random_bytes = vec![0u8; input.len()];
    get_random_values(&mut random_bytes);

    // Process each byte (XOR with random bytes for this example)
    for (i, &byte) in input.iter().enumerate() {
        output.push(byte ^ random_bytes[i]);
    }

    // Prepare to return a pointer to the allocated memory
    let output_len = output.len();

    // Create a new vector that includes the length and the data
    let mut result = Vec::with_capacity(4 + output_len);
    result.extend_from_slice(&(output_len as u32).to_le_bytes());
    result.extend_from_slice(&output);

    // Get the pointer to the result
    let result_ptr = result.as_mut_ptr();

    // Prevent the vector from being deallocated
    std::mem::forget(result);

    // Return the pointer
    result_ptr
}

// Allocates memory in the Wasm module and returns a pointer to it
// This function is called from JavaScript (see `wasm.alloc(inputData.length)`
// in the JS code)
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    buf.extend((0..len).map(|_| 0));
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf); // Prevent Rust from deallocating the memory
    ptr
}

// Deallocates memory in the Wasm module that was previously allocated by
// `alloc` This function is called from JavaScript (see `wasm.dealloc(inputPtr,
// inputData.length)` in the JS code)
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len); // Reconstruct and drop the Vec, freeing the memory
    }
}

// Declaration of the external function imported from JavaScript
// This allows the WASM module to call the `webcrypto_get_random_values`
// function defined in JavaScript
#[link(wasm_import_module = "env")]
extern "C" {
    fn webcrypto_get_random_values(ptr: *mut u8, len: usize);
}

// A safe wrapper for the imported JavaScript function
// This function is used as a custom random number generator for the `getrandom`
// crate
fn get_random_values(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    // Safety: webcrypto_get_random_values is assumed to be correctly linked to the
    // browser's crypto.getRandomValues method
    // See: https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues
    unsafe {
        webcrypto_get_random_values(dest.as_mut_ptr(), dest.len());
    };
    Ok(())
}

getrandom::register_custom_getrandom!(get_random_values);
