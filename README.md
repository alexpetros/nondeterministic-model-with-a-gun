# Non-deterministic Model With A Gun

A staged human-computer interaction about the power we give to our tools. Built with ChatGPT (for
now) and [Whisper.cpp](https://github.com/ggerganov/whisper.cpp).

This is the full version that we are going to instantiate inside a remote control robot, and it is
currently incomplete.

We demoed a prototype audio-only version of this at WordHack 3/16/23, which can be found at the [wordhack-demo tag](https://github.com/alexpetros/nondeterministic-model-with-a-gun/releases/tag/wordhack-demo).

Currently MacOS Apple Silicon only.

## Installation
* `cargo run` to run it
* `cargo build` to build it
* `cargo build -r` to build it in release mode

At this stage it listens and inserts your response into the interrogation script. I need to add back
the loop and the CLI commands.
