# Human Chat GPT
Talk to ChatGPT, and it talks back with a human voice.

Prototype; currently MacOS only.

## To Run
Requires a `.env` file with an OpenAI key in the exact format of:
```
OPENAI_API_KEY=youkeyhere
```

Also requires `sox` to run. `brew install sox` if you don't have it

`npm install` on first run

`node cli.js` to start the CLI

`node cli.js -f CONVERSATION_FILE` to start the CLI with a JSON conversation representation as the
starting point.

## While running
The conversation will start with an interrogation simulation. You can type in text, or talk to it by
pressing "Enter" and then speaking into the mic.

If you response before the speech is done it will talk over you so wait for it to finish.

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
