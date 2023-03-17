// import fs from 'node:fs'
import readline from 'node:readline/promises'
import { execSync } from 'node:child_process'

import Conversation from './conversation.js'
import * as simulations from './simulations.js'
import * as transcription from './transcription.js'

// const args = process.argv.slice(2);
// let history = args[0] === '-f' ? JSON.parse(fs.readFileSync(args[1])) : undefined

console.log(`
Welcome to the ChatGPT TTS.
Type your responses, or return an empty line and then speak into the microphone.`)

let conversation = new Conversation(simulations.gameshow)
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdin,
  terminal: false
})

while (!conversation.isOver()) {
  const line = await rl.question('')
  let userInput
  switch (line) {
    case '.dump':
      conversation.dumpHistoryToFile()
      break
    case '.h':
      conversation = new Conversation(simulations.gameshow)
      break
    case '.i':
      conversation = new Conversation(simulations.interrogation)
      break
    case '': {
      userInput = await transcription.listen()
      if (userInput.toLowerCase().includes('interrogation')) {
        conversation = new Conversation(simulations.interrogation)
        execSync('say affirmative')
        break
      } else {
        const response = await conversation.say(userInput)
        execSync(`say "${response.replaceAll('"', '')}"`)
        console.log(`> ${response}`)
        break
      }
    }
    default: {
      userInput = line
      const response = await conversation.say(userInput)
      execSync(`say "${response.replaceAll('"', '')}"`)
      console.log(`> ${response}`)
      break
    }
  }
}

conversation.playEndAudio()
