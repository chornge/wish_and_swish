#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;

use alloc::vec::Vec;
// use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;

#[allow(unused_imports)]
use embassy_nrf::peripherals::SAADC;

#[allow(unused_imports)]
use embassy_nrf::saadc::{self, ChannelConfig, Config, Saadc};

//use embassy_time::{Duration, Timer};
use embedded_alloc::LlffHeap as Heap;
use panic_probe as _;
// use rustpotter::Rustpotter;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize allocator BEFORE using it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    info!("Starting...");

    let _model = include_bytes!("../model.rpw");

    //let mut rustpotter = Rustpotter::new(model).unwrap();

    loop {
        let _audio_data = get_audio_data(); // Replace with actual audio data
        //if rustpotter.process_audio(audio_data).unwrap() {
        //on_wakeword_detected();
        //}
    }
}

fn get_audio_data() -> Vec<i16> {
    Vec::from([0; 160]) // 160 samples of silence
}

fn _on_wakeword_detected() {
    info!("Moving to next state...");
}
