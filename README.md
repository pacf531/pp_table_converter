# pp_table_converter

> Converts AMD powerplay tables from Windows registry files to Linux binary files for use on Linux.

[![License](http://img.shields.io/:license-mit-blue.svg?style=flat-square)](http://badges.mit-license.org)

## Getting Started

You may download the portable binary release that has been provided or build your own version.

Run the utility with the following command:

````shell
$ pp_table_converter input_powerplay_file.reg output_powerplay_file.bin
````

Where input_powerplay_file.reg is the Windows registry file containing the AMD powerplay table and output_powerplay_file.bin is the output of where the converted file will go.

## Building

You will need to install Rust and its accompanying utilities in order to build pp_table_converter. Please refer to https://rustup.rs/ for further details.
 
After you have installed Rust, you should run the following commands to clone the repository in a place of your choice:

````shell
$ git clone https://github.com/pacf531/pp_table_converter
$ cd pp_table_converter
$ cargo build --release 
````
The utility should be located in the target/release folder after the build is completed.

## Usage

To apply this AMD powerplay table binary file, you must run a Linux distribution with Linux kernel v4.4 or higher.

You may run the following command to apply the powerplay to your card:

````shell
$ cat output_powerplay_file.bin > /sys/class/drm/card0/device/pp_table
````

However, if you are running more than one GPU which are not the same brand or model on your system, Linux will create multiple card# where # is a numerical ID for each GPU on your system, but there is no human identifiable information from this so we will need to find out manually what GPU card# refers to.

You will need to first run the following command to get a list of all the DRM outputs and PCIe IDs for each output, which a sample output is shown below:

````shell
$ ls -l /sys/class/drm/card*
lrwxrwxrwx 1 root root 0 Nov 21 00:29 /sys/class/drm/card0 -> ../../devices/pci0000:00/0000:00:03.1/0000:1f:00.0/0000:20:00.0/0000:21:00.0/drm/card0
lrwxrwxrwx 1 root root 0 Nov 21 00:29 /sys/class/drm/card0-DP-1 -> ../../devices/pci0000:00/0000:00:03.1/0000:1f:00.0/0000:20:00.0/0000:21:00.0/drm/card0/card0-DP-1
lrwxrwxrwx 1 root root 0 Nov 21 00:29 /sys/class/drm/card0-DP-2 -> ../../devices/pci0000:00/0000:00:03.1/0000:1f:00.0/0000:20:00.0/0000:21:00.0/drm/card0/card0-DP-2
lrwxrwxrwx 1 root root 0 Nov 21 00:29 /sys/class/drm/card0-DP-3 -> ../../devices/pci0000:00/0000:00:03.1/0000:1f:00.0/0000:20:00.0/0000:21:00.0/drm/card0/card0-DP-3
lrwxrwxrwx 1 root root 0 Nov 21 00:29 /sys/class/drm/card0-HDMI-A-1 -> ../../devices/pci0000:00/0000:00:03.1/0000:1f:00.0/0000:20:00.0/0000:21:00.0/drm/card0/card0-HDMI-A-1
...
````
For each card#, you may see other outputs denoting the different display connectors available but they are not important. For each card# you see, extract the last 5 numbers and punctuation for each card. In the example above, this would be ````21:00.0````.

Then run the following command and the system will tell you which GPU has that identifier, as in the following sample output:

````shell
$ lspci | grep 21:00.0
21:00.0 VGA compatible controller: Advanced Micro Devices, Inc. [AMD/ATI] Vega 10 XTX [Radeon Vega Frontier Edition]
````

## Built With

* [rust-hex](https://github.com/KokaKiwi/rust-hex) - Used for numerical conversions for the powerplay tables

## Contributing

Since this is a one time utility, I do not expect to make further changes to the code itself or maintain this. If you want to make further changes, you are welcome to fork this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.