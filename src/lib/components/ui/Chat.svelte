<script lang="ts">
	import * as Alert from '$lib/components/ui/alert'
	import { lastModel, openai, openaiPrompt, openaiTemp } from '$lib/openai'
	import { chatHistory, type DexieMessage } from '$lib/text.store.js'
	import { md } from '$lib/utils'
	import DOMPurify from 'dompurify'
	import type { ChatCompletionChunk, ChatCompletionMessageParam } from 'openai/resources/index.mjs'
	import type { Stream } from 'openai/streaming.mjs'
	import { onMount, tick } from 'svelte'
	import ChatMessage from './ChatMessage.svelte'
	import SelectConversation from './SelectConversation.svelte'
	import SelectModel from './SelectModel.svelte'
	import TextInput from './TextInput.svelte'
	import { liveQuery } from 'dexie'
	import NewConversation from './NewConversation.svelte'
	import Settings from './Settings.svelte'

	export let id: number
	let pending = false
	let streaming = ''
	let stream: Stream<ChatCompletionChunk> | undefined
	let scrollContainer: HTMLDivElement

	$: history = liveQuery<DexieMessage[] | undefined>(() =>
		chatHistory.messages.where({ cid: id }).toArray()
	)

	const onSubmit = async (query: string) => {
		if (pending) {
			return
		}
		const cid = id
		pending = true
		const copy: ChatCompletionMessageParam[] = [...($history ?? [])]

		try {
			await chatHistory.messages.add({ cid, content: query, role: 'user' } as DexieMessage)
			copy.push({
				role: 'user',
				content: query
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
				messages:
					$openaiPrompt.length > 0
						? [
								{
									role: 'system',
									content: $openaiPrompt
								},
								...copy
							]
						: copy,
				stream: true,
				temperature: $openaiTemp
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
		const onfocus = () => {
			scrollContainer.style.overflow = 'scroll'
		}
		const onblur = () => {
			scrollContainer.style.overflow = 'clip'
		}

		window.addEventListener('keydown', handleKeyDown)
		window.addEventListener('focus', onfocus)
		window.addEventListener('blur', onblur)
		return () => {
			window.removeEventListener('keydown', handleKeyDown)
			window.removeEventListener('focus', onfocus)
			window.removeEventListener('blur', onblur)
		}
	})
</script>

<div bind:this={scrollContainer} class="flex flex-grow flex-col gap-2 overflow-scroll px-4 pb-4">
	{#if !$history?.length}
		<div class="flex h-full flex-col items-center justify-center">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="lucide lucide-zap mx-auto stroke-muted-foreground"
				><path
					d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"
				/></svg
			>
			<p class="text-center text-sm text-muted-foreground">No messages yet</p>
		</div>
	{/if}
	{#each $history ?? [] as message}
		<ChatMessage {message} />
	{/each}
	{#if streaming}
		<Alert.Root class={'group inline-block w-fit max-w-[95%] p-1 px-2'}>
			<Alert.Description class="md block rounded-sm px-1">
				{@html DOMPurify.sanitize(md.render(streaming), { ADD_ATTR: ['target'] })}
			</Alert.Description>
		</Alert.Root>
	{/if}
</div>

<div class="z-10 flex flex-col items-start gap-1 px-4">
	<TextInput {onSubmit} {pending} />
	<div class="flex w-full flex-row justify-between">
		<div class="flex flex-row gap-1">
			<NewConversation />
			<SelectConversation />
			<SelectModel />
		</div>
		<Settings />
	</div>
</div>
