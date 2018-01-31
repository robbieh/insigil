
# Insigil

<meta property="og:image" content="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/insigil.png"/>
<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/insigil.png" height=320 width=320> <img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/theming-1.png" height=320 width=320>


### Because looking at your data should be fun

Insigil is designed to turn static or streaming data into an animated circular visualization. While inteded to be a useful utility - sometimes a quick histogram is just what I need! - Insigil is designed to be visually pleasing and, yes, a bit geeky.

## Modes and Invocation

Currently, Insigil can draw historgrams, sets of gauges, and text taken from stdin or text files. I have been using bash's [process substitution](http://tldp.org/LDP/abs/html/process-sub.html) to generate data streams.

### Scheme

#### Bar ring: -br

Show a set of numbers as a sort of round bar chart:

```
echo "1 2 3 4 5 4 3 2 1 2 3 4 5 6 5 4 3 4 5 6 7 6 5 4 3 2 1 2 3 2 1 2 3 4 3 4 5 6 7 6 5 4 3 2 1"| tr -s ' ' \\n > numbers
insigil -hr numbers
```
<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/bar-example.png" height=160 width=160>

#### Histogram ring: -hr

Show a set of numbers as a histogram:

```
echo "1 2 3 4 5 4 3 2 1 2 3 4 5 6 5 4 3 4 5 6 7 6 5 4 3 2 1 2 3 2 1 2 3 4 3 4 5 6 7 6 5 4 3 2 1"| tr -s ' ' \\n > numbers
insigil -hr numbers
```
<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/histogram-example.png" height=160 width=160>

#### Gauge set ring: -gr

Display the system's load average for the last 1, 5, and 15 mintues as three gauges:

```insigil -gr <( while true; do cut -f 1-3 -d ''  /proc/loadavg | tr -d . ; sleep 1; done )```

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/gauge-example.png" height=160 width=160>

#### Text ring: -tr

Just show some static text:

```insigil -tr <( echo "...--->>>|Insigil|<<<---...") ```

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/text-example.png" height=160 width=160>

#### Combinations

The flags can be combined, and each new one creates a new ring inside the previous one.

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

### Bugs and Limitations

Plenty.

