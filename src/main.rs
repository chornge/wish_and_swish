#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
// use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;

#[allow(unused_imports)]
use embassy_nrf::peripherals::SAADC;

#[allow(unused_imports)]
use embassy_nrf::saadc::{self, ChannelConfig, Config, Saadc};

//use embassy_time::{Duration, Timer};
use panic_probe as _;
// use rustpotter::Rustpotter;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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
