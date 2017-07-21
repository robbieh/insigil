
# Insigil

<img src="https://raw.githubusercontent.com/robbieh/insigil/master/docs/screenshots/insigil.png" height=320 width=320>

### Because looking at your data should be fun

Insigil is designed to turn static or streaming data into an animated circular visualization. While inteded to be a useful utility - sometimes a quick histogram is just what I need! - Insigil is designed to be visually pleasing and, yes, a bit geeky.

## Modes and Invocation

Currently, Insigil can draw historgrams, sets of gauges, and text taken from stdin or text files. I have been using bash's [process substitution](http://tldp.org/LDP/abs/html/process-sub.html) to generate data streams.

### Scheme

#### Histogram ring: -hr

Show a set of numbers as a histogram:

```
echo "1 2 3 4 5 4 3 2 1 2 3 4 5 6 5 4 3 4 5 6 7 6 5 4 3 2 1 2 3 2 1 2 3 4 3 4 5 6 7 6 5 4 3 2 1"| tr -s ' ' \\n > numbers
insigil -hr numbers
```

#### Gauge set ring: -gr

Display the system's load average for the last 1, 5, and 15 mintues as three gauges:

```insigil -gr <( while true; do cut -f 1-3 -d \  /proc/loadavg | tr -d . ; sleep 1; done )```

#### Text ring: -tr

Just show some static text:

```insigil -tr <( echo "...--->>>|Insigil|<<<---...") ```
