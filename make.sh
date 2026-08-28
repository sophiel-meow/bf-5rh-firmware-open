cargo build --release 
cargo objcopy --release -- -O binary bf5rh-fw.bin
../../Baofeng-UV-5RM-5RH-RE/reverse_engineering/uv5rm-wrap-tool/uv5rm-wrap-tool -m wrap -i bf5rh-fw.bin -o bf5rh-fw.BF -f
python tools/flash.py bf5rh-fw.BF /dev/ttyUSB0

