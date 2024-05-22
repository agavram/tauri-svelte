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
	import { liveQuery } from 'dexie'
	import NewConversation from './NewConversation.svelte'

	export let id: number
	let pending = false
	let streaming = ''
	let stream: Stream<ChatCompletionChunk> | undefined

	$: history = liveQuery<DexieMessage[] | undefined>(() =>
		chatHistory.messages.where({ cid: id }).toArray()
	)

	const onSubmit = async (query: string) => {
		if (pending) {
			return
		}
		const cid = id
		pending = true
		const copy = [...($history ?? [])]

		try {
			copy.push({
				role: 'user',
				content: query,
				cid,
				id: await chatHistory.messages.add({ cid, content: query, role: 'user' } as DexieMessage)
			})

			copy.length === 1 &&
				(async () => {
					const title = await openai.chat.completions.create({
						model: $lastModel.lastUsedId,
						messages: [
							...copy,
							{
								role: 'system',
								content:
									'You are tasked with generating a title for the above conversation.' +
									'Generate one in 4 words or fewer. Do NOT respond to the user since you are an outside observer'
							}
						]
					})

					chatHistory.chats.add({ id: cid, title: title.choices[0].message.content ?? '' })
				})()

			stream = await openai.chat.completions.create({
				model: $lastModel.lastUsedId,
				messages: copy,
				stream: true
			})

			for await (const chunk of stream) {
				if (cid != id) {
					return
				}
				streaming += chunk.choices[0]?.delta?.content ?? ''
			}

			await chatHistory.messages.add({ cid, content: streaming, role: 'assistant' } as DexieMessage)
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
	{#each $history ?? [] as message}
		<ChatMessage {message} />
	{/each}
	{#if streaming}
		<Alert.Root class={'group inline-block w-fit max-w-[90%] p-1 px-2'}>
			<Alert.Description class="md block rounded-sm px-1">
				{@html DOMPurify.sanitize(md.render(streaming))}
			</Alert.Description>
		</Alert.Root>
	{/if}
</div>

<div class="z-10 flex flex-col items-start gap-1 px-4">
	<TextInput {onSubmit} {pending} />
	<div class="flex flex-row gap-1">
		<NewConversation />
		<SelectConversation />
		<SelectModel />
	</div>
</div>
