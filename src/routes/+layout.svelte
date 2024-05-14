<script lang="ts">
	import '../app.css'
	import Header from './Header.svelte'
	import { invoke } from '@tauri-apps/api/tauri'
	import { appWindow } from '@tauri-apps/api/window'

	invoke('init_spotlight_window')

	const handleEscape = (event: KeyboardEvent) => {
		if (event.key === 'Escape') {
			event.preventDefault()
			invoke('hide_spotlight')
		}
	}
	window.addEventListener('keydown', handleEscape)
</script>

<main class="h-screen flex flex-col">
	<button
		class="h-5 w-full cursor-default"
		on:mousedown={() => {
			appWindow.startDragging()
		}}
	></button>
	<slot />
</main>
