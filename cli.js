// import fs from 'node:fs'
import readline from 'node:readline'
import { exec } from 'node:child_process'

import Conversation from './conversation.js'
import * as simulations from './simulations.js'
import * as transcription from './transcription.js'

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdin,
  terminal: false
})

// const args = process.argv.slice(2);
// let history = args[0] === '-f' ? JSON.parse(fs.readFileSync(args[1])) : undefined

console.log(`
Welcome to the ChatGPT TTS.
Type your responses, or return an empty line and then speak into the microphone.`)

const conversation = new Conversation(simulations.interrogation)
rl.on('line', async (line) => {
  if (line.startsWith('.dump')) {
    return conversation.dumpHistoryToFile()
  }

  const userInput = line === '' ? await transcription.listen() : line
  const response = await conversation.say(userInput)

  exec(`say "${response.replaceAll('"', '')}"`)
  console.log(`> ${response}`)
})
