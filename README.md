# Keyboard Controller
This repo contains a command line program for controlling clevo keyboard backlight

## Required Setup
The [Tuxedo Keyboard](https://github.com/tuxedocomputers/tuxedo-keyboard) Kernel module must be installed before either tool in this repo can be used.

Copy keyctl to /usr/bin `cp ./target/debug/keyctl /usr/bin/`
## Compilation 
The program can be compiled using cargo

`cargo build`

## Usage
```
Usage:
  keyctl [OPTIONS]

Change the backlight color of clevo keyboards

Optional arguments:
  -h,--help             Show this help message and exit
  -r,--read             Returns the current color and brightness values
  -c,--color COLOR      Set background color of the keyboard
  -b,--brightness BRIGHTNESS
                        Set brightness of the background of the keyboard
```

## Operating System
These programs have only been tested on my computer which is running KDE Neon but will work on other opterating systems that install sys files to the same location.