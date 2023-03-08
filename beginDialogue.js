//new voice-only method
export default async function userRespond (audioFilePath, userPreface = null, initialPrompt = this.history[0].content) {
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
