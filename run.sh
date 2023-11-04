#!/bin/bash

printf "Receiver:\n"
cargo run &
sleep 0.1
rustc "src/sender.rs" 
sleep 0.1
printf "Sender:\n"
./sender
