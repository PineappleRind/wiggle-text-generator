# wiggle-text-generator
Generate wiggles ... in Rust!

[Original implementation](https://github.com/PineappleRind/PineappleRind.github.io/blob/master/wiggle/worker.js) (old JavaScript code, needs optimization...)

## Build
1. [Have Rust installed](https://www.rust-lang.org/tools/install).
2. Build with `cargo build --release`.
3. Executable will be at `target/release/wiggle-text-generator`.

## Usage
Usage: `wiggle-text-generator text width height [ease] [bezier_params]`

`text`, `width`, and `height` are required, while `ease` and `bezier_params` are optional.

### `bezier_params` 
4 comma-separated values (no spaces). It only works if `ease` is `custom_bezier`.

If you don't want to fiddle with numbers, [here's a good visual editor](https://cubic-bezier.com).

### `ease`
One of `linear`, `sine`, `quadratic`, `cubic`, `exponential`, `quart_in`, `quart_out`, `custom_bezier`.

If using `custom_bezier`, [`bezier_params`](#bezier_params) is required.

## What's a wiggle?
This is a wiggle!
```
text
text
text
text
text
 text
 text
  text
  text
   text
    text
     text
      text
       text
        text
          text
           text
            text
             text
              text
               text
                text
                 text
                 text
                  text
                  text
                   text
                   text
                   text
                   text
                   text
                   text
                   text
                   text
                  text
                  text
                 text
                 text
                text
               text
              text
             text
            text
           text
          text
        text
       text
      text
     text
    text
   text
  text
  text
 text
 text
text
text
text
text
```
