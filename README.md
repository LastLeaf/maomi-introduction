# The Introduction Website Code of maomi

## build and run

To build and run in debug mode, execute:

```sh
wasm-pack build --target no-modules --dev && cargo run
```

To build release, execute:

```sh
wasm-pack build --target no-modules && cargo build --release
```

Visit `http://localhost:2080` for the website.

## Language

The language is controlled by `LANG` environment variable while compilation. Currently available languages:

* `C` for default (English);
* `zh_CN` for Simplified Chinese.

Specify `LANG=C` if there is compilation error about missing the translation file.

## Server-side Rendering

Server-side rendering is disabled by default.

To enable it, use feature `server-side-rendering` while compilation.
