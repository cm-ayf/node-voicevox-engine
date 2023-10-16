/// <reference types="node" />
import fs from 'fs/promises'

import test from 'ava'

import { AccelerationMode, Synthesizer, VoiceModel } from '../'

test('should synthesize', async (t) => {
  const synthesizer = await Synthesizer.new('@discordjs-japan', {
    accelerationMode: AccelerationMode.Cpu,
    cpuNumThreads: 1,
  })
  const model = await VoiceModel.fromPath('@discordjs-japan/sample.vvm')
  await synthesizer.loadVoiceModel(model)
  const query = await synthesizer.audioQuery('はろーわーるど！', 1)
  const wav = await synthesizer.synthesis(query, 1)
  fs.writeFile('test.wav', wav)
  t.true(wav.length > 0)
})
