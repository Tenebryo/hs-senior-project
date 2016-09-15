# hs-senior-project
The code and files for my high school senior project in Spring 2016. I built a robot that solves 3x3x3 puzzle cubes. The code is split into two parts, `control` is the desktop program that scans and solves the cube Due to time limitations, scanning is not fully implemented - as cameras had not been attached yet - and solving is done using the [min2phase library](https://github.com/cs0x7f/min2phase), although I eventually plan to implement both of these features in Rust. `executer` is arduino code that receives a series of encoded puzzle moves over a serial port and translates them to motor pulses.

A video of the robot at an early stage of progress can be found [here](https://www.youtube.com/watch?v=pKnuR1FhyfM).
