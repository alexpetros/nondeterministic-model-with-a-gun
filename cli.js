import readline from 'node:readline'
import { exec } from 'node:child_process'

import * as ai from './openai.js'

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdin,
  terminal: false
})

console.log('Welcome to the OpenAI TTS bot')
const conversation = new ai.Conversation()
rl.on('line', async (line) => {
  const response = await conversation.say(line)
  exec(`say ${response}`)
  console.log(response)
})
