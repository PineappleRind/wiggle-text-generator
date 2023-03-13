# wiggle-text-generator
Generate wiggles ... in Rust!

[Original implementation](https://github.com/PineappleRind/PineappleRind.github.io/blob/master/wiggle/worker.js) (old JavaScript code, needs optimization...)

## Build
1. [Have Rust installed](https://www.rust-lang.org/tools/install).
2. Build with `cargo build --release`.
3. Executable will be at `target/release/wiggle-text-generator`.

## Usage
Usage: `wiggle-text-generator text width height [ease]`

(`text`, `width`, and `height` are required, while `ease` is optional)

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
