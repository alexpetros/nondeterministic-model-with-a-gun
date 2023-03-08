import { simulationParams } from './simulationParams.js'
import { execSync } from 'node:child_process'
import config from './config.js'
import { Configuration, OpenAIApi } from 'openai'
import fs from 'fs'

const configuration = new Configuration({ apiKey: config.OPENAI_API_KEY })
const openai = new OpenAIApi(configuration)

function getAudio (fileCount, simulation) {
  const fileName = `./recordings/whisperTest_${simulation}_${fileCount++}.mp3`
  execSync(`sox -t coreaudio default ${fileName} silence 1 0.1 2% 1 2.0 2% > /dev/null 2>&1`)
  return fileName
}

async function getTranscript (fileName, userPreface, history) {
  const transcript = await openai.createTranscription(fs.createReadStream(fileName), 'whisper-1');
  let transcribedMessage = userPreface ? `${userPreface} ${transcript.data.text}`: transcript.data.text
  history.push({role: 'user', content: transcribedMessage})
  return transcribedMessage
}

async function getResponse (history) {
  const response = await openai.createChatCompletion({
    model: 'gpt-3.5-turbo',
    messages: history
  })
  const responseMessage = response.data.choices[0]
  if (!responseMessage?.message) throw new Error('OpenAi returned no responseMessage')

  const { content, role } = responseMessage.message
  history.push({ role, content })
  return content
}

export async function beginDialogue (submittedHistory, simulation, newRun) {
  if (!simulationParams[simulation]) return `No parameters found for simulation ${simulation}`
  console.log("SIMULATION HAS BEGUN")
  //if starting from scratch, reset the history to the initial prompt

  const newHistory = [{role : 'system', content : simulationParams[simulation].initialPrompt}]
  const history = newRun ? newHistory : submittedHistory
  // if (history.length === 1) {
  //   history[0] = {role : 'system', content : simulationParams[simulation].initialPrompt}
  // }

  const userPreface = simulationParams[simulation].userPreface || null

  let convoActive = true
  //save audio files sequentially
  let fileCount = 0 

  while (convoActive) {
    //get audio
    const audioFile = getAudio(fileCount++, simulation)
    //process audio
    const transcribedMessage = await getTranscript(audioFile, userPreface, history)
    console.log(transcribedMessage)
    //generate response
    const content = await getResponse(history)
    //say response
    let newContent = content.replaceAll('\'', '').replaceAll('"', '').replaceAll('`', '')
    execSync(`say "${newContent}"`)
    console.log(`> ${newContent}`)

    if (transcribedMessage.indexOf('end simulation as administrator') > -1) convoActive = false
    if (newContent.indexOf('[ACTION: ELIM]') > -1) {
      convoActive = false
      execSync(`afplay gunshot.mp3`)
    }
    //listen again
  }
}