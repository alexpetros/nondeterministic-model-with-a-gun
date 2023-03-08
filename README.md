# Human Chat GPT
Talk to ChatGPT, and it talks back with a human voice.

Prototype; currently MacOS only.

## To Run
Requires a `.env` file with an OpenAI key in the exact format of:
```
OPENAI_API_KEY=youkeyhere
```

requires `sox` to run. `brew install sox` if you don't have it

`npm install` on first fun

`node cli.js` to start the CLI

`node cli.js -f CONVERSATION_FILE` to start the CLI with a JSON conversation representation as the
starting point.

## While running
Just talk to it! If you response before the speech is done it will talk over you so wait for it to
finish.

At any point in the conversation, you can type `.dump` and the application will save your chat
history to a timestamped JSON file. The conversation can continue as normal after doing so.

## Adding Simulations
Simulations are objects that hold parameters for a given interaction. They are held in an array of objects that gets imported into `cli.js` from `simulationParams.js`. Currently, each situation is defined by a key (eg. `interrogation`). The value of that key is an object with three properties:
  - userPreface {string} : a string to be appended to the beginning of every user submission
  - endCondition {string} = a flag to search for in what you say, will end the convo by changing `convoActive` to false, exiting the while loop
  - initialPrompt {string} : resets the first index of the `this.history` array to whatever you want the initial prompt to be. Makes it easier to setup plot / guidelines.

