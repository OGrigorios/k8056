# k8056
Library crate that implements the communication with a k8056 relay card, written in Rust.

This library provides basic structs and enums that provide the specified ACSII instruction, needed to communicate with the K8056 8-CHANNEL RELAY CARD manufactured by Velleman
To send the instructions to the relay card, a serial library like [serialport](https://crates.io/crates/serialport) is needed.
