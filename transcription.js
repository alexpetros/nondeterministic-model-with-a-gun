import { execSync } from 'node:child_process'
import config from './config.js'
import { Configuration, OpenAIApi } from 'openai'
import fs from 'fs'

const configuration = new Configuration({ apiKey: config.OPENAI_API_KEY })
const openai = new OpenAIApi(configuration)

export async function listen () {
  const fileName = `./recordings/voicerecording-${(new Date).getTime()}.mp3`
  execSync(`sox -t coreaudio default ${fileName} silence 1 0.1 2% 1 2.0 2% > /dev/null 2>&1`)
  const transcript = await openai.createTranscription(fs.createReadStream(fileName), 'whisper-1');
  console.log('[DEBUG]' + transcript.data.text)
  return transcript.data.text
}
