# Yet Another Argument Parser

### Pronounced 'YAWP'. Barbarically if possible.

I'd use clap but I'm allergic to `unwrap`s and I'd use docopt but I'm allergic to funky macros.

It's a middle ground between [`clap`](https://clap.rs), which is really nicely customizable but seems to not use the type system very extensively, and [`docopt`](https://github.com/docopt/docopt.rs), which makes nice guarantees about the types of its results but is a little opaque and awkward to use (or be certain it's configured properly).

Overall, this should make parsing arguments relatively hard to fuck up.

## Example

See examples or unit tests throughout for examples of what is allowed

```rust
let mut b = false;
let mut c = 0u8;
let mut d: ::std::net::IpAddr = unsafe { ::std::mem::zeroed() };
Yaap::create()
    .get_flag(&mut b, Arg::from("quiet", "Suppress output").with_short("q"))
    .get_count(&mut c, Arg::from("verbose", "Set verbosity").with_short("v"))
    .get_val(&mut d, Arg::from("addr", "IP Address").is_required())
    .finish();
```

The following would be compile-time errors:

* Setting `is_required()` on a flag argument
* Providing a default for anything but a value

The following are runtime errors:

* Forgetting to call `.finish()`
* Nothing else

## Why tf do I need another arg parser?

Rust already has a handful of command-line argument parsers, but none that I particularly like. Clap feels big and frameworks based on parsing a help string make me uncomfortable. I always feel like I might have done something wrong because clap requires `unwrap`ping so many things and having a framework parse a helpe message feels so opaque and awkward to customize. 

I found myself implementing my own arg parsing and figured it made more sense to just maintain my own framework and add features as necessary instead of bouncing around trying to find a crate that offers everything I'm interested in.

TL;DR you don't

## Why to use this

* More stuff checked at compile-time
* Cool and novel

## Why not to use this

* Not as battle-tested as alternatives
* Stupid name

