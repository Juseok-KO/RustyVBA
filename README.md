# RustyVBA
Interface for calling Rust functions from Excel sheets.

It is composed of mainly following four parts:
1. (core) Common datatype that both Rust and VBA can understand.
2. (from_vba) Dynamic library to be loaded from VBA code.
3. (vba_script_generator) VBA code that loads the dynamic library and defines some VBA functions to communicate with the loaded library.
4. (dll_interface) An interface that allows Rust developers to write functions in Rust for the Excel.