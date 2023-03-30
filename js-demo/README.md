# Non-deterministic Model With A Gun (JavaScript Version)

A staged human-computer interaction about the power we give to our tools. Built with ChatGPT (for
now) and [Whisper.cpp](https://github.com/ggerganov/whisper.cpp).

This is the JavaScript prototype that we demoed at [WordHack](https://toddwords.com/wordhack/);
currently MacOS Apple Silicon only.

## Installation
1. Install `sox` with `brew install sox`.

2. `whisper` you will have to install manually but fortunately the installation instructions are easy.
Just clone the [whisper repository](https://github.com/ggerganov/whisper.cpp), build it, and make
sure that the resulting file is in your PATH somewhere as main. Let's say you have a `~/bin` folder
in your PATH:

```
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make
cp main ~/bin/whisper
```

3. Download the language model using the script in the [parent directory](vendor/models) and copy it
   to the [models directory here](js-demo/models).

4. Create a `.env` file with an OpenAI key in the exact format of:
```
OPENAI_API_KEY=youkeyhere
```

5. Then run `npm install`

I'll bundle this all eventually.

## To Run
`node cli.js` to start the CLI

`node cli.js -f CONVERSATION_FILE` to start the CLI with a JSON conversation representation as the
starting point.

## While running
The conversation will start with an interrogation simulation. You can type in text, or talk to it by
pressing "Enter" and then speaking into the mic.

If you response before the speech is done it will talk over you, so wait for it to finish.

At any point in the conversation, you can type `.dump` and the application will save your chat
history to a timestamped JSON file. The conversation can continue as normal after doing so.

## Adding Simulations
Simulations are objects that hold parameters for a given interaction. They have the following
attributes:
  - userPrefix {string} : a string to be appended to the beginning of every user submission
  - endCondition {string} : a flag that denotes the end of a conversation
  - endAudio {string} : audio that plays when the simulation ends
  - initialPrompt {string} : resets the first index of the `this.history` array to whatever you want
    the initial prompt to be. Makes it easier to setup plot / guidelines.
