# Yet Another Argument Parser

### Pronounced 'YAWP'. Barbarically if possible.

I like [`clap`](https://clap.rs) but I'm allergic to `unwrap`s and I'd use [`docopt`](https://github.com/docopt/docopt.rs) but I'm afraid of macro black magic.

This is meant to be a middle ground that is intuitively customizable and uses the type system. 

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

* Setting `required()` on a flag or subcommand argument
* Providing a default for anything except a value

The following are runtime errors:

* Forgetting to call `.finish()` (which is a compile warning)
* Nothing else

## Matching 

TODO table of matches

## Compile-time Mistakes

This framework is designed to check a lot at compile-time. 

### Requiring unrequirable arguments

In some cases certain arguments should be required. Though most common for Value arguments (e.g. `--val=42`), this might be desirable for other argument types as well. But it doesn't make sense for a flag to be required, as a flag argument only has two values: true (present) and false (absent).

```rust
let mut b = false;
Yaap::create()
    .get_flag(&mut b, ArgM::new("long", "binary flag").required())
    .finish();
```

results in 

```
error[E0277]: the trait bound `yaap::arg::types::flag::FlagArg: yaap::arg::Requirable` is not satisfied
```

### Providing a default value

Count and List argument types have values associated with omission (`0` and `vec![]` respectively), but a Value argument doesn't necessarily have a default value; instead, one can be provided.

TODO: uhhh *should* other arg types have default values??

### Forgetting `.finish()`

The `.finish()` function is necessary to check for errors while parsing and correctness is not guaranteed without it. Omitting it causes a runtime panic as well as a compile-time warning.

```
warning: unused `yaap::Yaap` which must be used: Remember to call `.finish()`
```

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

