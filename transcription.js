import fs from 'fs'
import { execSync } from 'node:child_process'

import { Configuration, OpenAIApi } from 'openai'
import config from './config.js'

const configuration = new Configuration({ apiKey: config.OPENAI_API_KEY })
const openai = new OpenAIApi(configuration)

export async function listen () {
  const fileName = `./recordings/voicerecording-${(new Date).getTime()}.mp3`
  execSync(`sox -t coreaudio default ${fileName} silence 1 0.1 2% 1 2.0 2% > /dev/null 2>&1`)
  const transcript = await openai.createTranscription(fs.createReadStream(fileName), 'whisper-1');
  const response = transcript.data.text
  console.log('[DEBUG]' + response)
  return response
}
