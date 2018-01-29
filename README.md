# riscan-pro

Read information from RiSCAN PRO project files.

[![Build Status](https://secure.travis-ci.org/gadomski/riscan-pro.svg?branch=master)](https://travis-ci.org/gadomski/riscan-pro)
[![riscan-pro](https://docs.rs/riscan-pro/badge.svg)](https://docs.rs/riscan-pro)

[RiSCAN Pro](http://www.riegl.com/products/software-packages/riscan-pro/) is software developed by [Riegl](http://riegl.com/) for [terrestrial LiDAR scanning](https://en.wikipedia.org/wiki/Lidar#Terrestrial_lidar).
RiSCAN Pro stores most project metadata, e.g. calibration and transformation matrices, in a xml file inside of the RiSCAN Pro project directory.
This is a Rust library and executable for reading these xml files and extracting the good bits.

**This project is not created by Riegl and no support from them is provided or implied.
Please do not contact Riegl about this software.**

## Installing the binary

To use **riscan-pro** from the command line, you'll need to [get Rust](https://www.rust-lang.org/en-US/install.html).
Once you've got that, installation is simple:

```bash
cargo install riscan-pro
```

In case there's changes on Github that haven't been pushed to crates.io yet, you can also install straight from Github:

```bash
cargo install --git https://github.com/gadomski/riscan-pro
```

## Using the binary

As of this writing, the binary can do three things.

### 1. Print some project information as json

This can be useful in case you want in ingest the project information downstream and don't want to parse all that icky xml:

```
riscan-pro path/to/myproject json
```

### 2. Save all the SOP matrices to `.dat` files

Exporting the SOP matrices from a giant project is a pain, so this command does it all in one step:

```
riscan-pro path/to/myproject sop where/to/save/the/files
```

Note that each file will be named after its scan position, with a `.dat` extension.
To only export "frozen" SOPs, use the `--frozen` flag.

### 3. Save the POP matrix to a `.dat` files

Just like the `sop` subcommand, `pop` saves the POP matrix to a file, named after the project and with a `.dat` extension:

```
riscan-pro path/to/myproject pop where/to/save/the/file
```
