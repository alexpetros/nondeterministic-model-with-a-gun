import fs from 'node:fs'
import { Configuration, OpenAIApi } from 'openai'
import config from './config.js'
import { execSync } from 'node:child_process'

const configuration = new Configuration({ apiKey: config.OPENAI_API_KEY })
const openai = new OpenAIApi(configuration)

export default class Conversation {
  constructor (history) {
    this.history = history || [
      { role: 'system', content: 'You are a helpful assistant.' }
    ]
  }

  async say (userMessage) {
    this.history.push({ role: 'user', content: userMessage })
    const response = await openai.createChatCompletion({
      model: 'gpt-3.5-turbo',
      messages: this.history
    })
    const responseMessage = response.data.choices[0]
    if (!responseMessage?.message) throw new Error('OpenAi returned no responseMessage')

    const { content, role } = responseMessage.message
    this.history.push({ role, content })
    return content
  }
  
  //new voice-only method
  async userRespond (audioFilePath, userPreface = null, initialPrompt = this.history[0].content) {
    // console.log("in userRespond", audioFilePath)
    if (this.history.length === 1) {
      this.history[0] = {role : 'system', content : initialPrompt}
    }
    //get the trasnscript from the new audio file
    // execSync(`hear -d -i ${audioFilePath} > ./transcribed_text.txt`)
    const transcript = await openai.createTranscription(fs.createReadStream(audioFilePath), 'whisper-1');
    let transcribedMessage = userPreface ? `${userPreface} ${transcript.data.text}`: transcript.data.text
    console.log(transcribedMessage)
    
    //push it to history
    this.history.push({role: 'user', content: transcribedMessage})
    // console.log(transcribedMessage)
    // console.log("before response")
    //generate a response
    const response = await openai.createChatCompletion({
      model: 'gpt-3.5-turbo',
      messages: this.history
    })
    // console.log("past response")
    //copied all of this from `say`
    const responseMessage = response.data.choices[0]
    if (!responseMessage?.message) throw new Error('OpenAi returned no responseMessage')

    const { content, role } = responseMessage.message
    this.history.push({ role, content })
    return [ transcribedMessage, content ]
  }

  dumpHistoryToFile () {
    const dump = JSON.stringify(this.history)
    const unixTime = (new Date()).getTime()
    fs.writeFileSync(`./chat-history-dump-${unixTime}.json`, dump)
    console.log('[System] Succesfully dumped chat history')
  }

}
