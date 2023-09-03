#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use nrf_modem::{SystemMode, ConnectionPreference};
use embassy_nrf::interrupt;

use nrf9160_hal::pac as pac;
use nrf9160_hal::pac::NVIC;

/* Required by nrf-modem */
use tinyrlibc as _;

mod heap;
mod logging;

use log::{error, info};
use rtt_target::{rprint, rtt_init_default, rprintln};

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
#[allow(non_snake_case)]
fn EGU1() {
    nrf_modem::application_irq_handler();
    cortex_m::asm::sev();
}

// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
#[allow(non_snake_case)]
fn IPC() {
    nrf_modem::ipc_irq_handler();
    cortex_m::asm::sev();
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    heap::init();

    let mut channels = rtt_init_default!();

    // Logging
    logging::init(channels.up.0);

    info!("<<<<< Booting >>>>>");

    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Enable the modem interrupts
    unsafe {
        NVIC::unmask(pac::Interrupt::EGU1);
        NVIC::unmask(pac::Interrupt::IPC);
        cp.NVIC.set_priority(interrupt::EGU1, 4 << 5);
        cp.NVIC.set_priority(interrupt::IPC, 0 << 5);
    }

    info!("Trying to init modem");
    let result = nrf_modem::init(SystemMode {
        lte_support: true,
        lte_psm_support: false,
        nbiot_support: true,
        gnss_support: true,
        preference: ConnectionPreference::None,
    })
    .await.unwrap();
    
    // match result {
    //     Ok(val) => {
    //         info!("Succesfully initialized");
    //     }
    //     Err(err) => {
    //         error!("Error: {:?}", err);
    //     }
    // }

    let response = nrf_modem::send_at::<64>("AT+CGMI\r\n").await.unwrap();
    
    // Convert the ArrayString to a string slice and print it
    let string_slice: &str = response.as_str();
    // Convert the ArrayString into a String
    rprint!(string_slice);

    let mut cmd: Vec<u8> = Vec::new();
    rprint!("\r\nAT > ");
    loop {
        let mut buf = [0u8; 128];
        let bytes_read = channels.down.0.read(&mut buf);
        if bytes_read == 0 {
            continue;
        }
        let s = core::str::from_utf8(&buf[..bytes_read]).unwrap();
        cmd.extend(&buf[..bytes_read]);

        rprint!(s);
        if s.contains("\r") {
            cmd.push(b'\n');
            let at_string = core::str::from_utf8(&cmd).unwrap_or("Invalid UTF-8");

            let response = nrf_modem::send_at::<64>(at_string).await.unwrap();
            // Convert the ArrayString to a string slice and print it
            let string_slice: &str = response.as_str();
            // Convert the ArrayString into a String
            rprint!(string_slice);
            cmd.clear();
            rprint!("\nAT > ");
        }
    }
}
