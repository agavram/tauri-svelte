<script lang="ts">
	import * as Select from '$lib/components/ui/select'
	import { chatHistory, openedDialog, selectedConversation } from '$lib/text.store'
	import { cn } from '$lib/utils'
	import { liveQuery } from 'dexie'
	import { onMount, tick } from 'svelte'
	import { z } from 'zod'

	const setOpen = async (b: boolean) => {
		b ? openedDialog.set('SELECT-CONVERSATION') : openedDialog.set('')
		const content = document.querySelector('#select-conversation-trigger')
		await tick()
		if (content instanceof HTMLButtonElement) {
			b ? content.focus() : content.blur()
		}
	}
	$: open = 'SELECT-CONVERSATION' === $openedDialog

	let conversations = liveQuery(() => chatHistory.chats.reverse().toArray())

	onMount(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.metaKey && event.key === 'y') {
				setOpen(!open)
			}
		}

		window.addEventListener('keydown', handleKeyDown)
		return () => window.removeEventListener('keydown', handleKeyDown)
	})
</script>

<Select.Root
	{open}
	onOpenChange={setOpen}
	onSelectedChange={(e) => selectedConversation.set(z.number().parse(e?.value))}
>
	<Select.Trigger
		id="select-conversation-trigger"
		class="h-auto gap-1 p-1 text-xs hover:bg-muted focus:outline-none"
		on:click={(e) => console.log(e)}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="12"
			height="12"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="lucide lucide-history"
			><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" /><path d="M3 3v5h5" /><path
				d="M12 7v5l4 2"
			/></svg
		>
		History
	</Select.Trigger>
	<Select.Content class="max-h-80 overflow-y-scroll" sameWidth={false}>
		<Select.Label>Chat History</Select.Label>
		<Select.Separator />
		{#if $conversations === undefined}
			<Select.Item disabled value="Loading..."></Select.Item>
		{:else if $conversations.length === 0}
			<Select.Item class="data-[highlighted]:bg-background" disabled value="No chat history"
			></Select.Item>
		{:else}
			<div class="flex flex-col">
				{#each $conversations as conversation}
					<Select.Item
						class={cn(
							'relative inline w-full max-w-60 flex-row place-items-center overflow-hidden text-ellipsis whitespace-nowrap pr-2',
							$selectedConversation === conversation.id && 'border border-muted-foreground'
						)}
						value={conversation.id}
						label={conversation.title}
					></Select.Item>
				{/each}
			</div>
		{/if}
	</Select.Content>
</Select.Root>
