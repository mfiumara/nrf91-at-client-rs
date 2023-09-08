#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use nrf_modem::{SystemMode, ConnectionPreference};
use embassy_nrf::interrupt;

use nrf9160_hal::pac as pac;
use nrf9160_hal::pac::NVIC;

/* Required by nrf-modem */
extern crate tinyrlibc;
// extern crate panic_rtt_target;
use defmt_rtt_target as _;
// use defmt_rtt as _;
use panic_probe as _;

mod heap;

use defmt::{info};
use rtt_target::{rtt_init_default, rprint};

extern crate alloc;

use alloc::vec::Vec;

// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
#[allow(non_snake_case)]
fn IPC() {
    nrf_modem::ipc_irq_handler();
    cortex_m::asm::sev();
}

// #[defmt::timestamp]
// fn timestamp() -> u64 {
//     static COUNT: AtomicUsize = AtomicUsize::new(0);
//     // NOTE(no-CAS) `timestamps` runs with interrupts disabled
//     let n = COUNT.load(Ordering::Relaxed);
//     COUNT.store(n + 1, Ordering::Relaxed);
//     n as u64
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    heap::init();

    let channels = rtt_init_default!();

    // Logging
    defmt_rtt_target::init(channels.up.0);

    info!("<<<<< Booting >>>>>");

    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Enable the modem interrupts
    unsafe {
        NVIC::unmask(pac::Interrupt::IPC);
        cp.NVIC.set_priority(interrupt::IPC, 0 << 5);
    }

    info!("Trying to init modem");
    let result = nrf_modem::init(SystemMode {
        lte_support: false,
        lte_psm_support: false,
        nbiot_support: false,
        gnss_support: false,
        preference: ConnectionPreference::None,
    })
    .await.unwrap();
        // .unwrap();

    let response = nrf_modem::send_at::<64>("AT+CGMI\r\n").await.unwrap();
    
    // Convert the ArrayString to a string slice and print it
    let string_slice: &str = response.as_str();
    // Convert the ArrayString into a String
    info!("{}",string_slice);

    let mut cmd: Vec<u8> = Vec::new();
    let mut input = channels.down.0;
    loop {
        let mut buf = [0u8; 128];
        let bytes_read = input.read(&mut buf);
        if bytes_read == 0 {
            continue;
        }
        let s = core::str::from_utf8(&buf[..bytes_read]).unwrap();
        cmd.extend(&buf[..bytes_read]);

        // Echo on
        if s.contains("\r") {
            cmd.push(b'\n');
            let at_string = core::str::from_utf8(&cmd).unwrap_or("Invalid UTF-8");

            let response = nrf_modem::send_at::<64>(at_string).await.unwrap();
            // Convert the ArrayString to a string slice and print it
            let string_slice: &str = response.as_str();
            // Convert the ArrayString into a String
            // rprint!(string_slice);
            cmd.clear();
            // rprint!("\r\nAT > ");
        }
    }
}
