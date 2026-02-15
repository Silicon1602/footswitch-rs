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
git clone https://github.com/Silicon1602/footswitch-rs
cd footswitch-rs
cargo build --release
```

Optionally, install the application using:

```bash
cargo install
```

## Usage
When running `footpedal-rs --help`, the following instructions are shown:

```bash
footswitch-rs 0.1.0
Dennis Potter <dennis@dennispotter.eu>

USAGE:
    footswitch-rs [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    append    Append a key, a modifier, or a string to one or more pedals
    clear     Clear the value of one or more pedals
    help      Prints this message or the help of the given subcommand(s)
    list      Prints a table of all possible keys
    read      Read from the footpedal
    set       Set a key or a mousebutton to one or more pedals
```

When trying to read or modify the foot pedal (subcommands: `append`, `clear`, `read`, and `set`), footswitch-rs must be executed as super user.

### Reading from the foot pedal
To read the foot pedal without writing any settings, the subcommand `read` should be used. The help function `footswitch-rs read --help` yields te following information (omitted redundant information):

```bash
USAGE:
    footswitch-rs read [FLAGS] [OPTIONS]

FLAGS:
    -a, --all        Read all pedals

OPTIONS:
    -p, --pedal <pedals>...    Specify specific pedals. Possible values: [0 | 1 | 2]
```

In other words, to acquire the settings of all pedals, run: `footswitch-rs read --all`. To acquire only information of specific pedals (e.g., 1 and 2), run: `footswitch-rs read -p 1 2`.

### Writing to the foot pedal
#### The `list` subcommand
The subcommand `footswitch-rs list` returns a table with all possible key names and the translated value that will be written to the foot pedal. The provided key names can be used together with `footswitch-rs set key` or `footswitch-rs append key`. The help function `footswitch-rs list --help` yields te following information (omitted redundant information):

```bash
USAGE:
    footswitch-rs list --columns <columns>

OPTIONS:
    -c, --columns <columns>    Specify the number of columns of the table
```

The columns option is mandatory and sets the number of columns the table will use. That way, a user can make sure that the table fits his or her terminal.

#### The `set` subcommand
The help function `footswitch-rs set --help` yields te following information (omitted redundant information):

```bash
USAGE:
    footswitch-rs set <SUBCOMMAND>

SUBCOMMANDS:
    help             Prints this message or the help of the given subcommand(s)
    key              Set a key value to one or more pedals
    mousebutton      Set a mousebutton (left/right/middle/double) to one or more pedals
    mousemovement    Set X, Y, and W movement of the mouse pointer for one or more pedals
```

For every set subcommand, one or more pedals must be defined according to: `footswitch-rs set <SUBCOMMAND> -p [0 | 1 | 2]`. Furthermore, the value that should be set must be provided with the `-i` option (in case of `key` and `mousebutton`) or the `-x`, `-y`, and `-w` options (in case of `mousemovement`). More information on the possible values in the table below:

| Subcommand      | Option(s)            | Possible values for option(s)          |
| --------------- | -------------------- | -------------------------------------- |
| `key`           | `-i`                 | any key name from `footswitch-rs list` |
| `mousebutton`   | `-i`                 | [left \| right \| middle \| double]    |
| `mousemovement` | `-x`, `-y`, and `-w` | any integer between -128 and 127       |

When using `set` on a pedal, its content will be overwritten.

##### Examples

```bash
# Set pedal 0 to 'a'
sudo footswitch-rs set key -p 0 -i a

# Set pedal 0 to 'a', pedal 1 to 'b', pedal 2 to '<esc>'
sudo footswitch-rs set key -p 0 -i a -p 1 -i b -p 2 -i esc

# Set pedal 0 to a double click
sudo footswitch-rs set mousebutton -p 0 -i double

# Set pedal 0 to a mouse movement of x=100, y=100, w=0
sudo footswitch-rs set mousemovement -p 0 -x 100 -y 100 -w 0
```

#### The `append` subcommand
The help function `footswitch-rs append --help` yields te following information (omitted redundant information):
```bash
USAGE:
    footswitch-rs append <SUBCOMMAND>

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    key         Append a key value to one or more pedals
    modifier    Append a modifier (ctrl/shift/alt/win) to one or more pedals
    string      Append a string to one or more pedals
```

For every set subcommand, one or more pedals must be defined according to: `footswitch-rs append <SUBCOMMAND> -p [0 | 1 | 2]`. Furthermore, the value that should be set must be provided with the `-i` option.

| Subcommand | Option(s) | Possible values for option(s)          |
| ---------- | --------- | -------------------------------------- |
| `key`      | `-i`      | any key name from `footswitch-rs list` |
| `modifier` | `-i`      | [ctrl \| shift \| alt \| win]          |
| `string`   | `-i`      | any string                             |

When using `append` on a pedal, the value that is defined in `-i` will be appended to the pedal's existing content. However, not all combinations are possible! For example, a modifier cannot be appended to a key that is set with `footswitch-rs append key`, but only to a key that is set with `footswitch-rs set key` (see [Error: Invalid combination of options!](error-invalid-combination-of-options)).

##### Examples
```bash
# Set several keys to the same pedal
sudo footswitch-rs append key -p 0 -i a -p 0 -i b -p 0 -i c

# Set ctrl + alt + del to a pedal
sudo footswitch-rs set key -p 0 -i del
sudo footswitch-rs append modifier -p 0 -i ctrl -p 0 -i alt

# Append a string to a pedal
sudo footswitch-rs append string -p 0 -i 'Hello World'
```

### Clearing pedals
In contrast to the original implementation on [rgerganov/footswitch](https://github.com/rgerganov/footswitch), this implementation does not clear foot pedals if they are not explicitly set during a write operation. To clear the configuration of a pedal, a separate function has to be explicitly invoked. The help function `footswitch-rs clear --help` yields te following information (omitted redundant information):

```bash
USAGE:
    footswitch-rs clear [OPTIONS]

OPTIONS:
    -p, --pedal <pedal>...    Specify pedal(s) to clear: [0 | 1 | 2]
```

Thus, to clear pedals (e.g., 1 and 2), run: `footswitch-rs clear -p 1 2`.

## Common problems
### Error: Invalid combination of options!
This error can only occur with the `append` subcommand. footswitch-rs can set the foot pedal into four different, valid modes:

```rust
enum Type {
    Unconfigured = 0,
    Key = 1,
    Mouse = 2,
    MouseKey = 3,
    String = 4,
}
```

When appending something to a pedal, the application checks if this is valid. Obviously, this has not been the case when this error occurs. 

This error often occurs when somenone tries to add a modifier (`footswitch-rs append modifier`) to one or more keys that were added with `footswitch-rs append key` instead of `footswitch-rs set key`. In the former case, the type of the key(s) is set to `Type::String`. In the latter case it is set to `Type::Key`. However, to append a modifier, `Type::Key` is required.

In the previously mentioned case, a solution would be to clear the pedal, set a key with `footswitch-rs set`, and subsequently append the modifier(s).

### Hardware issues

Several people have reported misbehaviors with the PCsensor footswitch due to hardware issues. If the pedal is continuously sending a keypress without being pressed, then most probably some of the elements do not make good contact with the PCB. Follow [the instructions that were posted on original footswitch repository](https://github.com/rgerganov/footswitch/issues/26#issuecomment-401429709) to verify and and fix this.
