# C-LOAD

C-load is a very simple build system and package manager for C, like `cargo` for `rust`.

!!! Still in development !!!

## Installation
You will have to compile C-load yourself, this is not that hard. 

You should have `cargo` installed, if you don't have it, install it.

Go into the c-load project, compile it with this command.

```bash
$ cargo build
```

Now you have compiled the c-load source code.
You can find the executable in `target/debug/c-load`

## How to use c-load
To initialize a project, type this command

```bash
$ c-load init
```

This will create a `src/` directory with in there `main.c`. It will also create a `.gitignore` file
and a `bin/` folder. The compiled binaries will be stored in this folder.

--- 

To build the project, use the build command
```bash
$ c-load build
```

This will compile all source files into 1 binary, you can choose what compiler to use in the config file.

---

To run the project, use the run command
```bash
$ c-load run
```

This will build and run the project.

## C-load configuration
To configure c-load you need to make a `.clconfig` file in the root directory.

The config file should be written in JSON format.

--

To change the compiler (default is `clang`)
```json
{
    "compiler": "gcc"
}
```

--

To change the name of the output executable (default is `main.out`)
```json
{
    "output": "a.out"
}
```

--

To set compiler flags (by default no flags are set)
```json
{
    "c_flags": ["-pedantic", "-Wall"]
}
```

-- 

Get verbose output (by default is set to`false`)
```json
{
    "verbose": true
}
```