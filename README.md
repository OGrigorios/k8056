# k8056
Library crate that implements the communication with a k8056 relay card, written in Rust.

This library provides basic structs and enums that provide the specified ACSII instruction, needed to communicate with the K8056 8-CHANNEL RELAY CARD manufactured by Velleman
To send the instructions to the relay card, a serial library like [serialport](https://crates.io/crates/serialport) is needed.

Working example using serialport library
```
use serialport;
use k8056::uart::{Command, Idx};
use std::thread;
use std::time::Duration;  

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 2_400)
        .timeout(Duration::from_millis(10))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open()
        .expect("Failed to open port");
    // Just a bunch of Commands to show how to initialize them
    let cmd = Command::Byte(0x1C);
    let cmd = Command::Emergency;
    let cmd = Command::Force;
    let cmd = Command::Display;
    let cmd = Command::Address(Idx::new(2));
    port.write(&cmd.to_bytes(1)).expect("Write failed!");
    for i in 1..9 {
        let cmd = Command::Toggle(Idx::new(i));
        port.write(&cmd.to_bytes(1)).expect("Write failed!");
        thread::sleep(Duration::from_millis(1000));
    }

    for i in (1..9).rev() {
        let cmd = Command::Toggle(Idx::new(i));
        port.write(&cmd.to_bytes(1)).expect("Write failed!");
        thread::sleep(Duration::from_millis(1000));
    }
}
```
