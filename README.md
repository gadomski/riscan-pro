# riscan-pro

Crack open RiSCAN Pro xml project files.

[![Build Status](https://secure.travis-ci.org/gadomski/riscan-pro.svg?branch=master)](https://travis-ci.org/gadomski/riscan-pro)
[![riscan-pro](https://docs.rs/riscan-pro/badge.svg)](https://docs.rs/riscan-pro)

[RiSCAN Pro](http://www.riegl.com/products/software-packages/riscan-pro/) is software developed by [Riegl](http://riegl.com/) for [terrestrial LiDAR scanning](https://en.wikipedia.org/wiki/Lidar#Terrestrial_lidar).
RiSCAN Pro stores most project metadata, e.g. calibration and transformation matrices, in a xml file inside of the RiSCAN Pro project directory.
This is a Rust library for reading these xml files and extracting the good bits.

**This project is not created by Riegl and no support from them is provided or implied.
Please do not contact Riegl about this software.**

This library is not complete, as there's lots of project components that aren't supported.
This was developed for a specific purpose (colorizing points and transforming them) and so far hasn't been developed much beyond that.
