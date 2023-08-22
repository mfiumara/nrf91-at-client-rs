//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;

extern crate alloc;
use alloc::vec;
use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use futures_lite::future;
use async_std::task;

async fn say_hi() {
    info!("Hello, world!");
}

mod kernel;
mod logging;
use log::{debug, error, info, trace, warn};

#[entry]
fn main() -> ! {
    kernel::init();

    // Logging
    logging::init();
    debug!("Debug logging!");
    trace!("Trace logging!");
    info!("Info logging!");
    warn!("Warn logging!");
    error!("Err logging!");

    // Runtime async-std requires for task API feature alloc enabled.
    // This depends on event listener which seems to depend on std

    // Runtime smol
    // let val = future::block_on(async {
    //     1 + 2
    // });

    // Growable array allocated on the heap
    let xs = vec![0, 1, 2];

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
