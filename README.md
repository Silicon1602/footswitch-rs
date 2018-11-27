***This application is still under development. Currently, the `append` subcommand shows some problems.***

# footswitch-rs

This repository contains a C to Rust translation of the GitHub repository [rgerganov/footswitch](https://github.com/rgerganov/footswitch). Although I use footswitch-rs myself to manage my footswitch, the main purpose of its development was to practice Rust. Since I do not possess Apple hardware, I did not test this application on MacOS (unlike [rgerganov](https://github.com/rgerganov) did with the original footswitch). However, with some modifications, footswitch-rs should run on MacOS as well.

This utility supports [PCSensor](http://www.pcsensor.com/) foot switches with the following `vendorId:productId` combinations:

* `0c45:7403`
* `0c45:7404`
* `413d:2107`

footswitch-rs can also be used together with [VIM Clutch](https://github.com/alevchuk/vim-clutch).


## Installation

First, make sure that Rust is installed on your computer.

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then, clone this repository and use cargo to build it:

```bash
git clone https://git.dennispotter.eu/Dennis/footswitch-rs
cd footswitch-rs
cargo build --release
```

Optionally, install the application using:

```bash
cargo install
```

## Usage
Information about the usage (of a subcommand) can be found by running:

```bash
footswitch-rs <subcommand> --help
```

or [on the utilities wiki page](https://git.dennispotter.eu/Dennis/footswitch-rs/wiki).

## Hardware issues

Several people have reported misbehaviors with the PCsensor footswitch due to hardware issues. If the pedal is continuously sending a keypress without being pressed, then most probably some of the elements do not make good contact with the PCB. Follow [the instructions that were posted on original footswitch repository](https://github.com/rgerganov/footswitch/issues/26#issuecomment-401429709) to verify and and fix this.
