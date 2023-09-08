define bmp_init
	tar ext /dev/cu.usbmodem98B724951
	mon version
	mon swd_scan
	attach 1
end

define load_spm
	# load spm/spm-1.5.1.elf
	load spm/zephyr.hex
end

define factory_reset_thingy91
	load ~/Downloads/thingy91_fw_2023-06-01_0677b099/img_app_bl/debug/thingy91_asset_tracker_v2_debug_2023-06-01_0677b099.hex
end

define load_application
	load target/thumbv8m.main-none-eabihf/debug/nrf91-at-client-rs
	file target/thumbv8m.main-none-eabihf/debug/nrf91-at-client-rs
	mon reset
	mon swd_scan
	attach 1
	mon rtt
end







bmp_init
load_application
# load_spm
# factory_reset_thingy91
