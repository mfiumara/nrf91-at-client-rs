tar ext /dev/cu.usbmodem98B724951
mon version
mon swd_scan
attach 1

# Flash application firmware
load target/thumbv8m.main-none-eabihf/debug/nrf91-at-client-rs
# Flash the SPM
# load spm/spm-1.5.1.elf

# Pre-load our application symbols
file target/thumbv8m.main-none-eabihf/debug/nrf91-at-client-rs
# Reset the device to boot through SPM
mon reset
mon swd_scan
attach 1
mon rtt
continue
