# PHPer

A library that allows us to write PHP extensions using pure Rust and using safe Rust whenever possible.

(一个让我们可以使用纯Rust并且尽可能使用safe Rust写PHP扩展的库。)

***Now the peojct is still under development.***

## Usage

First you have to install `cargo-generate`:

```bash
cargo install cargo-generate
```

Then create a PHP extension project from the [template](https://github.com/jmjoy/phper-ext-skel.git):

```bash
cargo generate --git https://github.com/jmjoy/phper-ext-skel.git
```

## Notice

Now the library don't support `ZTS`, the template is using `thread_local!` instead.

Version `0.1.x` will be a preview version.

