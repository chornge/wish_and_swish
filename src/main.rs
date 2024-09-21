mod engine;

use engine::listen_for_keyword;

fn main() {
    println!("Starting Wish n' Swish...");

    listen_for_keyword();
}
