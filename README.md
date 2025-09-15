# RustyVBA
Interface for calling Rust functions from Excel sheets.

It is composed of mainly following four parts:
1. (core) Common datatype that both Rust and VBA can understand.
2. (from_vba) Dynamic library to be loaded from VBA code.
3. (vba_script_generator) VBA code that loads the dynamic library and defines some VBA functions to communicate with the loaded library.
4. (dll_interface) An interface that allows Rust developers to write functions in Rust for the Excel.

How to start

1. Compile the "from_vba" into the dynamic library.
2. Place the compiled dynamic library to any directory, and create a folder where custom dlls would be placed.
3. Create VBA script using "vba_script_generator": It will ask for the directories for the dynamic library and folder.
4. Import the script from Excel.
5. Create custom functions in the form of .dll and place them into the specified folder.