<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { appWindow } from '@tauri-apps/api/window'
	import { onMount } from 'svelte'
	import '../app.css'

	invoke('init_spotlight_window')

	onMount(() => {
		const handleEscape = (event: KeyboardEvent) => {
			if (event.key === 'Escape') {
				event.preventDefault()
				invoke('hide_spotlight')
			}
		}
		window.addEventListener('keydown', handleEscape)

		return () => {
			window.removeEventListener('keydown', handleEscape)
		}
	})
</script>

<main class="flex h-screen flex-col rounded-md border-2 border-zinc-900">
	<button
		class="min-h-5 w-full cursor-default"
		on:mousedown={() => {
			appWindow.startDragging()
		}}
	></button>
	<div
		class="mx-auto flex h-full w-full max-w-screen-md flex-grow flex-col justify-between overflow-hidden pb-1"
	>
		<slot />
	</div>
</main>
