import fs from 'node:fs'
import { Configuration, OpenAIApi } from 'openai'
import config from './config.js'

const configuration = new Configuration({ apiKey: config.OPENAI_API_KEY })
const openai = new OpenAIApi(configuration)

export default class Conversation {
  constructor (params) {
    this.userPrefix = params.userPrefix
    this.endCondition = params.endCondition
    this.history = [{ role: 'system', content: params.initialPrompt }]
  }

  async say (userMessage) {
    const messageWithPrefix = this.userPrefix + userMessage
    this.history.push({ role: 'user', content: messageWithPrefix })
    const { role, content } = await getNextMessage(this.history)

    this.history.push({ role, content })
    return content
  }

  isOver () {
    const lastMessage = this.history.at(-1)?.content
    return lastMessage.includes(this.endCondition)
  }

  dumpHistoryToFile () {
    const dump = JSON.stringify(this.history)
    const unixTime = (new Date()).getTime()
    fs.writeFileSync(`./chat-history-dump-${unixTime}.json`, dump)
    console.log('[System] Succesfully dumped chat history')
  }
}

async function getNextMessage (history) {
  const response = await openai.createChatCompletion({ model: 'gpt-3.5-turbo', messages: history })
  const message = response.data.choices[0].message
  if (!message) throw new Error('OpenAi returned no responseMessage', response.data)
  return message
}
