import fs from 'node:fs'

import { Configuration, OpenAIApi } from 'openai'
import config from './config.js'

const configuration = new Configuration({ apiKey: config.OPENAI_API_KEY })
const openai = new OpenAIApi(configuration)

export class Conversation {
  constructor () {
    this.history = [
      {role: 'system', content: 'You are a helpful assistant.'},
    ]
  }

  async say (userMessage) {
    this.history.push({role: 'user', content: userMessage})
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

  dumpHistoryToFile () {
    const dump = JSON.stringify(this.history)
    const unixTime = (new Date()).getTime()
    fs.writeFileSync(`./chat-history-dump-${unixTime}.json`, dump)
    console.log('[System] Succesfully dumped chat history')
  }
}
