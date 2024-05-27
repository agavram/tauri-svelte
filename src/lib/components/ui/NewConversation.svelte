<script>
	import * as Tooltip from '$lib/components/ui/tooltip'
	import { selectedConversation } from '$lib/text.store'
	import Button from './button/button.svelte'
</script>

<Tooltip.Root>
	<Tooltip.Trigger>
		<Button
			variant="ghost"
			class="h-auto gap-1 border border-input p-1 hover:bg-muted hover:text-foreground focus:outline-none text-xs"
			on:click={() => selectedConversation.set(new Date().getTime())}
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
				class="lucide lucide-plus"><path d="M5 12h14" /><path d="M12 5v14" /></svg
			>
			New Chat
		</Button>
	</Tooltip.Trigger>
	<Tooltip.Content
		class="z-10 flex flex-row gap-2 bg-muted px-2 text-foreground shadow shadow-background"
	>
		<kbd
			class="shadow-kbd inline-flex size-5 items-center justify-center rounded-sm border border-muted bg-background text-sm"
			>⌘</kbd
		>
		<kbd
			class="shadow-kbd inline-flex size-5 items-center justify-center rounded-sm border border-muted bg-background font-mono text-sm"
			>T</kbd
		>
	</Tooltip.Content>
</Tooltip.Root>
