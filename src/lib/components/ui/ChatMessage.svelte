<script lang="ts">
	import * as Alert from '$lib/components/ui/alert'
	import { chatHistory, type DexieMessage } from '$lib/text.store'
	import { cn, md } from '$lib/utils'
	import { writeText } from '@tauri-apps/api/clipboard'
	import DOMPurify from 'dompurify'
	import gsap from 'gsap'
	import { tick } from 'svelte'
	import Button from './button/button.svelte'

	export let message: DexieMessage

	let svg: SVGElement
	let edit = false
	let input: HTMLElement
	let copied = false
</script>

<Alert.Root
	class={cn(
		'group inline-block w-fit max-w-[95%] p-1 px-2',
		message.role === 'user' && 'place-self-end',
		edit && 'border-foreground'
	)}
>
	<Alert.Description class="md block rounded-sm px-1">
		{#if edit}
			<p
				class="edit my-[5px] w-full outline-none"
				on:blur={() => {
					edit = false
					chatHistory.messages.update(message.id, { content: message.content })
				}}
				contenteditable="plaintext-only"
				bind:innerText={message.content}
				bind:this={input}
			>
				{message.content}
			</p>
		{:else}
			{@html DOMPurify.sanitize(md.render(message.content), { ADD_ATTR: ['target'] })}
		{/if}
	</Alert.Description>
	<div
		class="invisible absolute right-0 z-10 scale-50 select-none opacity-0 transition ease-in-out group-hover:visible group-hover:scale-100 group-hover:opacity-100"
	>
		<Alert.Root class="p-1">
			<Alert.Description class="flex flex-row">
				<Button
					variant="ghost"
					class="relative h-auto p-2"
					on:click={async (event) => {
						try {
							await writeText(message.content)
						} catch (e) {}

						if (!copied) {
							copied = true
							setTimeout(() => (copied = false), 2000)
						}
					}}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="lucide lucide-copy"
					>
						{#if copied}
							<path class="animate-fadeInOut opacity-0 transition-opacity" d="m12 15 2 2 4-4" />
						{/if}
						<rect width="14" height="14" x="8" y="8" rx="2" ry="2" /><path
							d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
						/>
					</svg>
				</Button>
				<Button
					variant="ghost"
					class="h-auto p-2"
					on:click={async () => {
						edit = true
						await tick()
						input.focus()
					}}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="lucide lucide-pencil-ruler"
						><path d="m15 5 4 4" /><path
							d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13"
						/><path d="m8 6 2-2" /><path
							d="m2 22 5.5-1.5L21.17 6.83a2.82 2.82 0 0 0-4-4L3.5 16.5Z"
						/><path d="m18 16 2-2" /><path
							d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17"
						/></svg
					>
				</Button>
				<Button
					variant="ghost"
					class="h-auto p-2"
					on:click={() => chatHistory.messages.delete(message.id)}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="lucide lucide-trash-2"
						><path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
							d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
						/><line x1="10" x2="10" y1="11" y2="17" /><line x1="14" x2="14" y1="11" y2="17" /></svg
					>
				</Button>
			</Alert.Description>
		</Alert.Root>
	</div>
</Alert.Root>
