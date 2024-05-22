<script>
	import { selectedConversation } from '$lib/text.store'
	import Button from './button/button.svelte'
</script>

<Button
	variant="ghost"
	class="h-auto gap-1 border border-input p-1 hover:bg-muted focus:outline-none"
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
	<span class="text-xs font-normal">New Chat</span>
</Button>
