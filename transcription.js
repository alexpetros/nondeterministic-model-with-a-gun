import readline from 'node:readline/promises'
import { exec, execSync } from 'node:child_process'

const MODEL_FP = './models/ggml-base.en.bin'

export async function listen () {
  const fileName = `./recordings/voicerecording-${(new Date).getTime()}.wav`
  const recorder = exec(`sox -t coreaudio default -r 16000 -c 1 -b 16 ${fileName}`)

  const rl = readline.createInterface({ input: process.stdin, output: process.stdin, terminal: false })
  await rl.question('Recording. Press "enter" to stop\n')
  recorder.kill()

  const whisper = execSync(`whisper -nt -f ${fileName} -m ${MODEL_FP} 2> /dev/null`)
  const response = whisper.toString().trim()

  // TODO throw if the response is empty
  console.log('[DEBUG] ' + response)
  return response
}
