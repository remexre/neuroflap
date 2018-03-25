# neuroflap

A genetic algorithm building neural nets to play Flappy Bird.

The actual Flappy Bird game is rendered by Vulkan, and is playable by a human.

## Installing

Requires `rustc` 1.25 or later.

Run `cargo install` to install neuroflap with all modes.

To install fewer modes, instead run `cargo install --no-default-features --features MODES`, where `MODES` is a comma-separated list of:

 - `play` -- The human-playable game. Adds the `play` subcommand.
 - `simulate` -- The simulator. Adds the `sim` subcommand.
 - `train` -- The training program. Adds the `extract`, `new`, and `train` subcommands.

At least one mode must be provided.

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
