import OpenAI from 'openai'
import storedWritable from '@efstajas/svelte-stored-writable'
import z from 'zod'
import { get } from 'svelte/store'

export const openaiKeyZ = z.string()
export const openaiKey = storedWritable('openai-key', openaiKeyZ, '')

export const openaiPromptZ = z.string()
export const openaiPrompt = storedWritable('openai-prompt', openaiKeyZ, '')

export const openaiTempZ = z
	.number({ message: 'Temperature must be between 0 and 2' })
	.min(0, { message: 'Temperature must be between 0 and 2' })
	.max(2, { message: 'Temperature must be between 0 and 2' })
export const openaiTemp = storedWritable('openai-temp', openaiTempZ, 1)

export const lastModel = storedWritable(
	'last-model',
	z.object({
		lastUsedId: z.string()
	}),
	{
		lastUsedId: 'gpt-4o'
	}
)

export const openAiSchema = z.object({
	apiKey: openaiKeyZ,
	temperature: openaiTempZ,
	prompt: openaiPromptZ
})

export const openai = new OpenAI({
	dangerouslyAllowBrowser: true,
	apiKey: get(openaiKey)
})

openaiKey.subscribe((key) => {
	openai.apiKey = key
})
