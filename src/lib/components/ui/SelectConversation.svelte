<script lang="ts">
	import { Button } from '$lib/components/ui/button'
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu'
	import { chatHistory, selectedConversation } from '$lib/text.store'
	import { liveQuery } from 'dexie'
	import { cn } from '../../utils'

	let conversations = liveQuery(() => chatHistory.chats.reverse().toArray())
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button
			variant="ghost"
			class="flex h-auto flex-row justify-center gap-1 px-2 py-1 text-xs"
			builders={[builder]}
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
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="max-h-80 overflow-y-scroll">
		<DropdownMenu.Group>
			<DropdownMenu.Label>Chat History</DropdownMenu.Label>
			<DropdownMenu.Separator />
			{#if $conversations === undefined}
				<DropdownMenu.Item>Loading...</DropdownMenu.Item>
			{:else if $conversations.length === 0}
				<DropdownMenu.Item class="data-[highlighted]:bg-background">No chat history</DropdownMenu.Item>
			{:else}
				<div class="flex flex-col">
					{#each $conversations as conversation}
						<DropdownMenu.Trigger>
							<Button
								variant="ghost"
								class={cn(
									'w-full justify-start',
									$selectedConversation === conversation.id && 'border border-muted-foreground'
								)}
								on:click={() => selectedConversation.set(conversation.id)}
							>
								<span class="max-w-60 overflow-hidden text-ellipsis">
									{conversation.title}
								</span>
							</Button>
						</DropdownMenu.Trigger>
					{/each}
				</div>
			{/if}
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
