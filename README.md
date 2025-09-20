# Table of Contents
* [Description](#description)
* [Getting Started](#getting-started)
	* [Starting File](#starting-file)
	* [Compilation](#compilation)
* [More Resources](#more-resources)
* [Future Plans](#future-plans)

# Description
This project will eventually hold both the Unimal compiler and the portable assembly assembler, but for now it just holds the Portable Assembly Assembler 

# Getting Started
## Starting File
To begin, a portable assembly file is created simply by creating a new file, `main.pasm`, with the following text
```pasm
stt 10

set !1, 0, 1

end
```
which will create a new function ten bytes in size and set the first byte to `1`

## Compilation
To turn `main.pasm` into a Portable Binary file, just run `pacc /path/to/main.pasm`, and this will spit out a final `a.pbin` file.  
You also may specify the output file using the `--output` or `-o` command.

# More Resources
For a language reference, check out the attached Portable Assembly documentation. Thank you again for checking out this repo!

# Future Plans
 - [ ] Fully integrate floating point arithmetic
 - [ ] Add the portable interface, which will supply a basic cross-platform library
 - [ ] Integrate SDL support natively into `pasm` with frame buffering
