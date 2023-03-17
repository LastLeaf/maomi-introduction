# The Introduction Website Code of maomi

## build and run

To build and run in debug mode, execute:

```sh
wasm-pack build --target no-modules --dev && cargo run
```

Visit `http://localhost:2080` for the website.

To build release, execute:

```sh
wasm-pack build --target no-modules && cargo build --release
```

## Language

The language is controlled by `MAOMI_I18N_LOCALE` environment variable while compilation. Currently available languages:

* default (English);
* `zh_CN` for Simplified Chinese.

Specify `MAOMI_I18N_LOCALE=` if there is compilation error about missing the translation file.

## Server Side Rendering

Server-side rendering is disabled by default.

To enable it, use feature `server-side-rendering` while compilation.

```sh
wasm-pack build --target no-modules --features server-side-rendering && cargo build --release --features server-side-rendering
```

It seems that there might hit some rust compiler bugs. If so, disable the incremental compilation through `CARGO_INCREMENTAL=0` .
