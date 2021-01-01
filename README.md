# Stock Fetcher
- a simple stock fetcher cli written in rust.

# Build requirements
- gcc/g++/gcc-arm-linux-gnueabihf toolchain
    - sudo apt install gcc && sudo apt install g++ && sudo apt install gcc-arm-linux-gnueabihf
- windows toolchain
    - sudo apt install mingw-w64
- Rust
    - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
_ Rust targets
    - rustup target add armv7-unknown-linux-gnueabihf
    - rustup target add x86_64-pc-windows-gnu
    - rustup toolchain add stable-x86_64-pc-windows-gnu

# Mandatory config
- copy email_config.json.dev to working directory where the binary is executed or cargo root dir.
    - rename the file from email_config.json.dev -> email_config.json
- supply your own the email config.

# Building
## Build targets (windows, linux, arm7)
- make build
## Build target arm7
- make build_pi

# Running
## Linux
- ./target/release/stock-fetcher
## Windows
- ./target/x86_64-pc-windows-gnu/release/stock-fetcher
## Arm7
- ./target/armv7-unknown-linux-gnueabihf/release/stock-fetcher

# Deploying to rasberrypi
- make deploy_pi ip=IP_ADDRESS


# License
```MIT License

Copyright (c) 2020 infotamia

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.```
