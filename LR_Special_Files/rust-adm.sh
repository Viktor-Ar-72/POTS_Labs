sudo apt install curl gdb-multiarch libudev-dev pkg-config qemu-system-arm build-essential
echo 'SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0d28", MODE="0666", TAG += "uaccess"' | sudo tee /etc/udev/rules.d/50-microbit.rules
sudo udevadm control --reload-rules
sudo udevadm trigger




