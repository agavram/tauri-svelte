<script lang="ts">
	import Chat from '$lib/components/ui/Chat.svelte'
	import { onMount } from 'svelte'
	import { selectedConversation } from '$lib/text.store'

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.metaKey && event.key === 't') {
			selectedConversation.set(new Date().getTime())
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeyDown)
		return () => window.removeEventListener('keydown', handleKeyDown)
	})
</script>

<svelte:head>
	<title>Chat</title>
	<meta name="description" content="Chat with LLMs" />
</svelte:head>

<Chat id={$selectedConversation} />
