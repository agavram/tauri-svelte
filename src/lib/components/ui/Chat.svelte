<script lang="ts">
	import * as Alert from '$lib/components/ui/alert'
	import { pending } from '$lib/text.store.js'
	import { cn } from '$lib/utils'
	import DOMPurify from 'dompurify'
	import markdownit from 'markdown-it'
	import OpenAI from 'openai'
	import type { ChatCompletionMessageParam } from 'openai/resources/index.mjs'
	const md = markdownit()

	const openai = new OpenAI({
		apiKey: 'sk-proj-fpW9howG4wbGGDo2PDK7T3BlbkFJuTWWuQCMcq74AZ1GWl5O',
		dangerouslyAllowBrowser: true
	})

	let history: ChatCompletionMessageParam[] = []
	let streaming = ''

	pending.subscribe(async (query) => {
		if (!query) {
			return
		}
		try {
			history = [
				...history,
				{
					role: 'user',
					content: query
				}
			]

			const stream = await openai.chat.completions.create({
				model: 'gpt-4o',
				messages: history,
				stream: true
			})

			for await (const chunk of stream) {
				streaming += chunk.choices[0]?.delta?.content ?? ''
			}

			if (streaming) {
				history.push({
					role: 'assistant',
					content: streaming
				})
				history = [...history]
				streaming = ''
			}
		} finally {
			pending.set('')
		}
	})
</script>

<div class="flex flex-col gap-2 overflow-scroll px-4 pb-4">
	{#each history as { role, content }, i}
		<Alert.Root
			class={cn('inline-block w-fit max-w-[90%]', role === 'user' ? 'place-self-end' : '')}
		>
			<Alert.Description class="md">
				{@html DOMPurify.sanitize(md.render(content?.toString() ?? '', {}))}
			</Alert.Description>
		</Alert.Root>
	{/each}
	{#if streaming}
		<Alert.Root class="inline-block w-fit max-w-[90%]">
			<Alert.Description class="md">
				{@html DOMPurify.sanitize(md.render(streaming))}
			</Alert.Description>
		</Alert.Root>
	{/if}
</div>
