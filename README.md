# March Madness Predictor

## Description

A from scratch implementation of a genetic algorithm neural network used to generate following text in a shakespear sonnet.

Library used is something I've separated for use in other AI projects. It is maintained [here](https://github.com/blueOkiris/scratch_genetic).

## Build/Run

You just need the Rust build system, `cargo`

__Train:__

`cargo run --release`

The release is important because it adds a MAJOR performance boost

__Predict:__

`cargo run --release -- --predict=<initial line>`

Initial line has a max length of 100 characters

Example:
`cargo run --release -- --predict=From\ fairest\ creatures\ we\ desire\ increase`
