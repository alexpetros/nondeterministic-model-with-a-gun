# Human Chat GPT
Talk to ChatGPT, and it talks back with a human voice.

Prototype; currently MacOS only.

## To Run
Requires a `.env` file with an OpenAI key in the exact format of:
```
OPENAI_API_KEY=youkeyhere
```

`npm install` on first fun

`npm run start` to start the CLI

## While running
Just talk to it! If you response before the speech is done it will talk over you so wait for it to
finish.

At any point in the conversation, you can type `.dump` and the application will save your chat
history to a timestamped JSON file. The conversation can continue as normal after doing so.
