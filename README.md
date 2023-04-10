# Non-deterministic Model With A Gun

A staged human-computer interaction about the power we give to our tools. Built with GPT-4 (for
now) and [Whisper.cpp](https://github.com/ggerganov/whisper.cpp).

This is the full version that we are going to instantiate inside a remote control robot, and it is
currently incomplete.

We demoed a prototype audio-only version of this at WordHack 3/16/23, which can be found at the [wordhack-demo tag](https://github.com/alexpetros/nondeterministic-model-with-a-gun/releases/tag/wordhack-demo).

Currently MacOS Apple Silicon only.

## Installation
The first time you download, you will need to download models. You can do that with:
```
bash ./vendor/models/download-base-en.sh
```

During development:
* `cargo run` to run it
* `cargo build` to build it
* `cargo build -r` to build it in release mode

