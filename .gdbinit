tar ext /dev/cu.usbmodem98B724951
mon version
mon swd_scan
attach 1

# Flash application firmware
load target/thumbv8m.main-none-eabihf/debug/rs-rtos
# Flash the SPM
# file /Users/mfiumara/repos/nrf-sdk/nrf/samples/spm/build/zephyr/zephyr.elf
# load /Users/mfiumara/repos/nrf-sdk/nrf/samples/spm/build/zephyr/zephyr.elf

# Pre-load our application symbols
file target/thumbv8m.main-none-eabihf/debug/rs-rtos
# Reset the device to boot through SPM
mon reset
mon swd_scan
attach 1
mon rtt
continue
