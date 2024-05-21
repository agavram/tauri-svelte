<script lang="ts">
	import * as Alert from '$lib/components/ui/alert'
	import { lastModel, openai } from '$lib/openai'
	import { chatHistory, type DexieMessage } from '$lib/text.store.js'
	import { md } from '$lib/utils'
	import DOMPurify from 'dompurify'
	import type { ChatCompletionChunk } from 'openai/resources/index.mjs'
	import type { Stream } from 'openai/streaming.mjs'
	import { onMount } from 'svelte'
	import ChatMessage from './ChatMessage.svelte'
	import SelectConversation from './SelectConversation.svelte'
	import SelectModel from './SelectModel.svelte'
	import TextInput from './TextInput.svelte'

	export let id: number
	let pending = false
	let streaming = ''
	let stream: Stream<ChatCompletionChunk> | undefined

	let history: DexieMessage[] = []
	$: (async () => {
		const cid = id
		let messages: DexieMessage[] = []

		if ((await chatHistory.chats.where({ id: cid }).count()) === 1) {
			messages = await chatHistory.messages.where({ cid }).toArray()
		}
		history = []

		if (cid == id) {
			history = [...messages]
		}
	})()

	const onSubmit = async (query: string) => {
		if (pending) {
			return
		}
		const cid = id
		pending = true

		try {
			if (!history.length) {
				await chatHistory.chats.add({ id: cid, title: new Date(cid).toLocaleString() })
			}

			history = [
				...history,
				{
					role: 'user',
					content: query,
					id: await chatHistory.messages.add({
						cid,
						content: query,
						role: 'user'
					} as DexieMessage),
					cid
				}
			]

			history.length === 1 &&
				(async () => {
					const title = await openai.chat.completions.create({
						model: $lastModel.lastUsedId,
						messages: [
							...history,
							{
								role: 'system',
								content:
									'You are tasked with generating a title for the above conversation. Generate one in 4 words or fewer. Do NOT respond to the user as you are an outside observer'
							}
						]
					})

					chatHistory.chats.put({ id: cid, title: title.choices[0].message.content ?? '' })
				})()

			stream = await openai.chat.completions.create({
				model: $lastModel.lastUsedId,
				messages: history,
				stream: true
			})

			for await (const chunk of stream) {
				if (cid != id) {
					return
				}
				streaming += chunk.choices[0]?.delta?.content ?? ''
			}

			history = [
				...history,
				{
					role: 'assistant',
					content: streaming,
					id: await chatHistory.messages.add({
						cid,
						content: streaming,
						role: 'assistant'
					} as DexieMessage),
					cid
				}
			]
		} finally {
			streaming = ''
			pending = false
			stream = undefined
		}
	}

	onMount(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.metaKey && e.key === 'd') {
				stream?.controller.abort()
			}
		}

		window.addEventListener('keydown', handleKeyDown)
		return () => window.removeEventListener('keydown', handleKeyDown)
	})
</script>

<div class="flex flex-grow flex-col gap-2 overflow-scroll px-4 pb-4">
	{#each history as message}
		<ChatMessage {message} />
	{/each}
	{#if streaming}
		<Alert.Root class="inline-block w-fit max-w-[90%] p-1 px-3">
			<Alert.Description class="md">
				{@html DOMPurify.sanitize(md.render(streaming))}
			</Alert.Description>
		</Alert.Root>
	{/if}
</div>

<div class="z-10 flex flex-col items-start gap-1 px-4">
	<TextInput {onSubmit} {pending} />
	<div class="flex flex-row gap-1">
		<SelectConversation />
		<SelectModel />
	</div>
</div>
