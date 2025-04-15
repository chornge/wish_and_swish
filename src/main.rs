#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(c_variadic)]

extern crate alloc;

use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
#[allow(unused_imports)]
use embassy_nrf::saadc::Saadc;
use linked_list_allocator::LockedHeap;
use panic_probe as _;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[link(name = "pv_porcupine")]
unsafe extern "C" {
    pub fn pv_porcupine_init(
        access_key: *const u8,
        memory_buffer_size: usize,
        memory_buffer: *mut u8,
        num_keywords: i32,
        keyword_model_sizes: *const i32,
        keyword_models: *const *const u8,
        sensitivities: *const f32,
        handle: *mut *mut c_void,
    ) -> i32;

    pub fn pv_porcupine_process(
        handle: *mut c_void,
        pcm: *const i16,
        keyword_index: *mut i32,
    ) -> i32;

    pub fn pv_porcupine_delete(handle: *mut c_void);
}

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

const MEMORY_BUFFER_SIZE: usize = 512;
static mut MEMORY_BUFFER: [u8; MEMORY_BUFFER_SIZE] = [0; MEMORY_BUFFER_SIZE];

const ACCESS_KEY: &str = "${{ secrets.PORCUPINE_ACCESS_KEY }}";
const KEYWORD_MODEL: &[u8] = include_bytes!("../assets/model.ppn");

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize heap allocator
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
        unsafe {
            #[allow(static_mut_refs)]
            HEAP.lock().init(HEAP_MEM.as_mut_ptr().cast(), HEAP_SIZE);
        }
    }

    info!("Starting...");

    // Initialize Porcupine
    let mut handle: *mut c_void = ptr::null_mut();
    let keyword_model_size = KEYWORD_MODEL.len() as i32;
    let sensitivity: f32 = 0.5;

    let status = unsafe {
        pv_porcupine_init(
            ACCESS_KEY.as_ptr(),
            MEMORY_BUFFER_SIZE,
            #[allow(static_mut_refs)]
            MEMORY_BUFFER.as_mut_ptr(),
            1,
            &keyword_model_size,
            &KEYWORD_MODEL.as_ptr(),
            &sensitivity,
            &mut handle,
        )
    };

    if status != 0 {
        panic!("Failed to initialize Porcupine");
    }

    loop {
        let audio_data = get_audio_data();
        process_audio(handle, &audio_data);
    }
}

fn process_audio(handle: *mut c_void, audio_data: &[i16]) {
    let mut keyword_index: i32 = -1;

    let status = unsafe { pv_porcupine_process(handle, audio_data.as_ptr(), &mut keyword_index) };

    if status != 0 {
        panic!("Failed to process audio");
    }

    if keyword_index != -1 {
        info!("Wake word detected!");
        on_wakeword_detected();
    }
}

fn get_audio_data() -> Vec<i16> {
    Vec::from([0; 160])
}

fn on_wakeword_detected() {
    info!("Wake word detected! Moving to the next state...");
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    let layout = Layout::from_size_align(size, core::mem::align_of::<usize>()).unwrap();
    unsafe { HEAP.alloc(layout) as *mut c_void }
}

#[unsafe(no_mangle)]
pub extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    let total_size = nmemb * size;
    let ptr = malloc(total_size);
    if !ptr.is_null() {
        unsafe { ptr::write_bytes(ptr, 0, total_size) };
    }
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn free(_ptr: *mut c_void) {
    // No-op: Memory is not explicitly freed because this system uses a static allocator
    // and does not support dynamic deallocation. This is intentional for simplicity
    // and to avoid fragmentation in the embedded environment.
}

#[unsafe(no_mangle)]
pub extern "C" fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            let byte = *src.add(i);
            *dest.add(i) = byte;
            if byte == 0 {
                break;
            }
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub extern "C" fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            if c1 != c2 {
                return c1 as i32 - c2 as i32;
            }
            if c1 == 0 {
                break;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn strspn(s: *const u8, accept: *const u8) -> usize {
    let mut count = 0;
    unsafe {
        while *s.add(count) != 0 {
            let mut found = false;
            let mut i = 0;
            while *accept.add(i) != 0 {
                if *s.add(count) == *accept.add(i) {
                    found = true;
                    break;
                }
                i += 1;
            }
            if !found {
                break;
            }
            count += 1;
        }
    }
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsnprintf(
    _str: *mut u8,
    _size: usize,
    _format: *const u8,
    _args: ...
) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprintf(_str: *mut u8, _size: usize, _format: *const u8, ...) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vfprintf(_stream: *mut c_void, _format: *const u8, _args: ...) -> i32 {
    0
}

#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static mut _impure_ptr: *mut c_void = ptr::null_mut();
