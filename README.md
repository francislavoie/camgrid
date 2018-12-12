# Camgrid

A simple Rust app which watches a list of directories for updated images and displays them in a grid.

## Build and Run

- Install Rust using [rustup](https://rustup.rs/)

- `cargo run`

## Configuration

Camgrid uses a configuration file named `Camgrid.toml` which is assumed to be in the same directory in which the program was run. If no configuration file exists, one will be created when the application closes. The currently supported options are:

- `paths`: an array of directories to watch for updated images. For example:

    ```toml
    paths = [
        "F:\\camera\\cam1",
        "F:\\camera\\cam2",
        "F:\\camera\\cam3",
    ]
    ```
