import fs from 'node:fs'
import readline from 'node:readline'
import { exec } from 'node:child_process'
import { execSync } from 'node:child_process'
import Conversation from './conversation.js'
import { simulationParams } from './simulationParams.js'
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdin,
  terminal: false
})

const args = process.argv.slice(2);
const history = args[0] === '-f' ? JSON.parse(fs.readFileSync(args[1])) : undefined

console.log('Welcome to the ChatGPT TTS. Enter a line of text, and the AI will respond.')
// const conversation = new ai.Conversation()

// original method
// rl.on('line', async (line) => {
//   const response = await conversation.say(line)
//   exec(`say ${response}`)
//   console.log(response)
// })



//new method
//set simulation parameters in simulationParams
// userPreface : string = a string to be appended to the beginning of every user submission
// endCondition : string = a flag to search for in what you say, will end the convo by changing `convoActive` to false, exiting the while loop
// initialPrompt : resets the `this.history` property to whatever you want the initial prompt to be. Makes it easier to setup plot / guidelines.

async function runSimulation (simulation) {
  
  //set a holder variable to allow a convo to end
  let convoActive = true

  //save audio files sequentially
  let fileCount = 0 

  if (simulationParams[simulation]) {
    //add other simulations to simulationParams to add more cases
    if (simulation === 'interrogation') {
      while (convoActive) {
        const fileName = `./recordings/whisperTest_hostage_${fileCount++}.mp3`
        execSync(`sox -t coreaudio default ${fileName} silence 1 0.1 2% 1 2.0 2%`)
        // console.log("syncDOne")
        const [transcript, content] = await conversation.userRespond(fileName, simulationParams[simulation].userPreface, simulationParams[simulation].initialPrompt)
        if (transcript.indexOf('end simulation as administrator') > -1) convoActive = false
        // console.log(transcript)
        let newContent = content.replaceAll('\'', '').replaceAll('"', '').replaceAll('`', '')
        execSync(`say "${newContent}"`)
        console.log(newContent)
      }
    }
  }
  else console.log("Simulation does not exist")
}

// run simulation with desired input
runSimulation('interrogation')


const conversation = new Conversation(history)
rl.on('line', async (line) => {
  if (line.startsWith('.dump')) {
    return conversation.dumpHistoryToFile()
  }

  const response = await conversation.say(line)
  exec(`say "${response}"`)
  console.log(`> ${response}`)
})
