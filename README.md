# Brightness Control Program

A command-line program to control the brightness of the display backlight on systems using the `intel_backlight` driver.

## Features

- Adjust the brightness to a specific value or percentage
- Get the current brightness level
- Display usage information
- Display program version

## Prerequisites

- Linux-based operating system
- `intel_backlight` driver support
- Rust programming language (if compiling from source)

## Installation

You can either download the pre-built binary or compile the program from source.

### Pre-built Binary

1. Download the binary file for your system from the [Releases](https://github.com/jonasrdl/rbacklight/releases) page.
2. Place the binary in a location accessible from the command line.
3. Set the necessary permissions to execute the binary, if required.

### Build from Source

1. Install the Rust programming language by following the instructions at [rustup.rs](https://rustup.rs).
2. Clone the repository:

   ```shell
   git clone https://github.com/jonasrdl/rbacklight.git

3. Navigate to the project directory: `cd rbacklight`
4. Build the binary: `cargo build --release`
5. The compiled binary will be available in the `target/release/` directory.

### Usage
The program supports the following command-line options:

`rbacklight [OPTIONS] [VALUE]`   

OPTIONS:   
-h: Show usage information
-v: Show program version   

VALUE:   
Numeric value: Sets the brightness to the specified value.
Percentage value: Sets the brightness as a percentage of the maximum value.

### Examples:

Set the brightness to a specific value:
`rbacklight 50`

Set the brightness to a specific percentage:
`rbacklight 50%`

Get the current brightness level:
`rbacklight`