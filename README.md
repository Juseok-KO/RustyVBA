# RustyVBA
Interface for Calling Rust Functions from Excel Sheets

This project enables seamless intergration between Rust and Excel via VBA. It consists of four main components:
1. core : Defines common datatypes that can be understood by both Rust and VBA.
2. from_vba : A dynamic library that can be loaded from VBA code.
3. vba_script_generator : Generates VBA script that load the dynamic library and define functions to communicate with it.
4. dll_interface : Provides an interface for Rust developers to write functions that can be called from Excel.

Getting Started

1. Compile the `from_vba` crate into a dynamic libarary.
2. Place the compiled library in any directory, and create a folder where custom DLLs will be stored.
3. Generate the VBA script using `vba_script_generator`. It will prompt you to specify the paths to the dynamic library and the DLL folder.
4. Import the generated script into Excel.
5. Create custom functions in Rust, compile them into `.dll` files, and place them in the specified folder.