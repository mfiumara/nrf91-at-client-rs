# nrf91-at-client-rs 

This project implements a simple AT client over RTT to the nrf9160 LTE modem.
It is used as a learning project in debugging rust on the thingy91 using a blackmagic
debug probe.

## Dependencies

The following dependencies are included in this project:

- rtt-target & logging => Used for logging over RTT and getting input from RTT
- embassy => To provide an async run-time so we can await on functions
- nrf-modem & nrfxlib => These provide rust bindings into the nrf modem library

## SPM & Memory

Since we are talking to the LTE modem we need to boot up in secure mode. The SPM from nrf-sdk 1.5.1 takes care of this for us and jumps to our application.

The `memory.x` specifies our application to be located at the correct address.

## Hardware set-up 

I am using a Blackmagic debug probe in combination with a thingy91 as my hardware set-up. You can use a nrf9160 development kit as well or connect a JLink or other probe to the thingy91. For the BMP I am using the `.gdbinit` script to auto-load the spm and firmware to the target.

I'm using the built-in RTT device from the BMP to check my RTT output, essentially having the two following terminals open:

For launchihg the application:
```
arm-none-eabi-gdb
```

For checking the RTT output and providing RTT input:
```
screen /dev/tty.usbmodem98B724953
```

