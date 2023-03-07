import fs from 'node:fs'
import readline from 'node:readline'
import { exec } from 'node:child_process'

import Conversation from './conversation.js'

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdin,
  terminal: false
})

const args = process.argv.slice(2);
const history = args[0] === '-f' ? JSON.parse(fs.readFileSync(args[1])) : undefined

console.log('Welcome to the ChatGPT TTS. Enter a line of text, and the AI will respond.')
const conversation = new Conversation(history)
rl.on('line', async (line) => {
  if (line.startsWith('.dump')) {
    return conversation.dumpHistoryToFile()
  }

  const response = await conversation.say(line)
  exec(`say "${response}"`)
  console.log(`> ${response}`)
})
