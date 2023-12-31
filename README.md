# C-LOAD

C-load is a very simple build system and package manager for C, like `cargo` for `rust`.

!!! Still in development !!!

Sorry I'm not so good at writing documentation 😅

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

--

Ignore C files for compilation.
```json
{
    "ignore": [
        "src/example.c"
    ]
}
```

In this example `src/example.c` would just be ignored an not be compiled.

Important to know is to **always** use the whole path, so dont forget the `src`.

--

Change entry point, if you don't want `main.c` to be your entry point you can use this option.
```json
{
    "entry": "entry.c"
}
```

Now the main function will be in `entry.c` instead of `main.c`.


## Package System

### Create a package

To create a package you first need to make a `.clpackage` file in the your project directory, this file is in a JSON format.

The file should look something like the example shown below.

```json
{
    "name": "mypackage",
    "version": "0.1.0",
    "author": "your name",
    "license": "MIT",

    "headers": [
        "src/vector.h",
        "src/math.h",
    ],

    "sources": [
        "src/vector.c",
        "src/math.c"
    ]
}
```

It's important that you give the full path from the project directory. The header files will be copied over to the project where this package is included and the sources will be compiled to object files and stored in the project using this package.

Name, version, author and license aren't used yet, but will be in the future.

### Use a package

To use a package you just need to add it to your `.clconfig` file, like shown in the example below.

```json
{
    "packages": [
        "/Users/your-name/dev/my-package-project"
    ]
}
```

Here it's important to write the absolute path (this will change in the future so it's easier), and don't write a '/' at the end.

Now the object files will automaticaly be compiled into your executable, to use the header files you need to include them with the correct path, this is shown down below.

```c
#include "../packages/the-package-name/the-file.h"
```

There will be a lot of bugs in the code for sure so please report them.