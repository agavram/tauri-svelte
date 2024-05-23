<script lang="ts">
	import Chat from '$lib/components/ui/Chat.svelte'
	import { selectedConversation } from '$lib/text.store'
	import { onMount } from 'svelte'

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.metaKey && event.key === 't') {
			selectedConversation.set(new Date().getTime())
		}

		if (event.key === 'Escape') {
			const activeElement = document.activeElement
			if (activeElement && 'blur' in activeElement && typeof activeElement.blur === 'function') {
				activeElement.blur()
			}
			event.preventDefault()
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
