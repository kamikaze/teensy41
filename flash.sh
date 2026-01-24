cargo objcopy --release -- -O ihex target/teensy41.hex
teensy_loader_cli --mcu=TEENSY41 -w target/teensy41.hex
