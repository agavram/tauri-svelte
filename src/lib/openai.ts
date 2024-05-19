import OpenAI from 'openai'
import storedWritable from '@efstajas/svelte-stored-writable'
import z from 'zod'

export const openai = new OpenAI({
	apiKey: 'sk-proj-fpW9howG4wbGGDo2PDK7T3BlbkFJuTWWuQCMcq74AZ1GWl5O',
	dangerouslyAllowBrowser: true
})

export const lastModel = storedWritable(
	'last-model',
	z.object({
		lastUsedId: z.string().min(1)
	}),
	{
		lastUsedId: 'gpt-4o'
	}
)
