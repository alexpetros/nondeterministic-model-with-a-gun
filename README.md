# Human Chat GPT
Talk to ChatGPT, and it talks back with a human voice.

Prototype. Currently MacOS only.

## To Run
requires `sox` to run. `brew install sox` if you don't have it

`npm install` on first fun

`npm run start` to start the cli

## Adding Simulations
Simulations are objects that hold parameters for a given interaction. They are held in an array of objects that gets imported into `cli.js` from `simulationParams.js`. Currently, each situation is defined by a key (eg. `interrogation`). The value of that key is an object with three properties:
  - userPreface {string} : a string to be appended to the beginning of every user submission
  - endCondition {string} = a flag to search for in what you say, will end the convo by changing `convoActive` to false, exiting the while loop
  - initialPrompt {string} : resets the first index of the `this.history` array to whatever you want the initial prompt to be. Makes it easier to setup plot / guidelines.

