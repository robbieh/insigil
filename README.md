
# Insigil

<meta property="og:image" content="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/insigil.png"/>
<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/insigil.png" height=320 width=320> <img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/theming-1.png" height=320 width=320>


### Because looking at your data should be fun

Insigil is designed to turn static or streaming data into an animated circular visualization. While inteded to be a useful utility - sometimes a quick histogram is just what I need! - Insigil is designed to be visually pleasing and, yes, a bit geeky.

## Modes and Invocation

Currently, Insigil can draw historgrams, sets of gauges, and text taken from stdin or text files. I have been using bash's [process substitution](http://tldp.org/LDP/abs/html/process-sub.html) to generate data streams.

### Scheme

#### Bar ring: -br

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/bar-example.png" height=160 width=160>

Show a set of numbers as a sort of round bar chart:

```
echo "1 2 3 4 5 4 3 2 1 2 3 4 5 6 5 4 3 4 5 6 7 6 5 4 3 2 1 2 3 2 1 2 3 4 3 4 5 6 7 6 5 4 3 2 1"| tr -s ' ' \\n > numbers
insigil -hr numbers
```
#### Histogram ring: -hr

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/histogram-example.png" height=160 width=160>

Show a set of numbers as a histogram:

```
echo "1 2 3 4 5 4 3 2 1 2 3 4 5 6 5 4 3 4 5 6 7 6 5 4 3 2 1 2 3 2 1 2 3 4 3 4 5 6 7 6 5 4 3 2 1"| tr -s ' ' \\n > numbers
insigil -hr numbers
```

#### Gauge set ring: -gr

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/gauge-example.png" height=160 width=160>

Display the system's load average for the last 1, 5, and 15 mintues as three gauges:

```insigil -gr <( while true; do cut -f 1-3 -d ''  /proc/loadavg | tr -d . ; sleep 1; done )```

#### Text ring: -tr

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/text-example.png" height=160 width=160>

Just show some static text:

```insigil -tr <( echo "...--->>>|Insigil|<<<---...") ```

#### Combinations

The flags can be combined, and each new one creates a new ring inside the previous one.

```insigil -tr <( echo "...--->>>|Insigil|<<<---...") ```

### Theming

Create a ```$HOME/.insigil.colors.toml``` file with contents such as this:

```
[palette]
background = [0.85, 0.90, 0.99, 1.0]
primary = [0.14, 0.75, 0.92, 1.0]
secondary = [0.99, 0.99, 0.99, 1.0]
highlight = [0.79, 0.41, 0.83, 1.0]
```

The format is R, G, B, Alpha scaled from 0.0 to 1.0.

## changelog

* 0.1.1 - Added a real histogram. Updated to latest Piston libs.
* 0.1.0 - Initial version

### Building it

First, [install Rust](https://www.rust-lang.org/en-US/install.html)

Then roughly:

```
git clone https://github.com/robbieh/insigil
cd insigil
cargo build --release
./target/release/insigil
```

If you want the SDL2 or GLFW backend, try one of these instead. Though for me this displays at the wrong scale on a high def display. YMMV.

```
cargo build --release --no-default-features --features include_sdl2
cargo build --release --no-default-features --features include_glfw
```


### Bugs and Limitations

Plenty.

