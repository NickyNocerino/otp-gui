AUTHOR: NICKY NOCERINO

TIMEPSTAMP: Sun Jan 26 2025 20:09:42 GMT-0500 (Eastern Standard Time)

OTP GUI

A minimalistic gui for one time pad file encryption and decryption
built on the otp-exchange rust package

Instructions for use:

MAC and UBUNTU

1) open your terminal and install Rust
   - https://www.rust-lang.org/tools/install
   - 1.5) you may need to activate the rust enviornment with this command
       - source ~/.cargo/env
2) download this repository
   - this can be done by cloning (via https or ssh) or downloading a zip file
   - 2.5) if neccessary you can change the branch of otp-echange use to match
     pad's size in the Cargo.toml file
       - legal branches at this time are [main, 10MB, 1GB, 10GB]
3) navigate to the root directory of this repository in the terminal
4) build and run with cargo byt running this command
   - cargo run --release
5) after the project compiles and runs, a gui window will pop up
   - select your pad from anywhere in the machines file tree
   - select the file you wish to encrypt or decrypt
   - select a where you would like the result outputed
   - you can continue using remaining bytes of the pad
