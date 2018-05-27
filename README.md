# WONDROUS BINGBALL

It's Pong in Rust. It's got sound effects and a little bit of physics.

The sound effects were made with [sfxr](http://www.drpetter.se/project_sfxr.html).


# Building

Once you've [installed Rust](https://www.rust-lang.org/en-US/install.html), you
should be able to build Wondrous Bingball by cloning this repository and running

    cargo build

A fresh build will take a few minutes to compile all the dependencies, but after
that builds should take a few seconds or less.

The demo was built with stable Rust 1.26.


# Distributing

To share a build, you should build an optimized binary with

    cargo build --release

this will put distributable binaries in `./target/release`.

You may also have to distribute the SDL2 libary (as `SDL2.dll`
or `SDL1.so`) along with the binary (the game may not run if
the library isn't there in its directory). You can download
that file [here](https://www.libsdl.org/download-2.0.php).

You should also be able to cross-compile the game, but this isn't
something I've investigated in detail. You can see an example of
cross-compilation in the [rustup](https://blog.rust-lang.org/2016/05/13/rustup.html)
documentation.
