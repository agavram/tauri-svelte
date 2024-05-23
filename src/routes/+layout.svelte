<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { appWindow } from '@tauri-apps/api/window'
	import '../app.css'
	import 'highlight.js/styles/stackoverflow-dark.min.css'

	invoke('init_spotlight_window')
</script>

<main class="flex h-screen flex-col rounded-md border-2 border-zinc-900">
	<button
		class="min-h-5 w-full cursor-default"
		on:mousedown={() => {
			appWindow.startDragging()
		}}
	></button>
	<div
		class="mx-auto flex h-full w-full max-w-screen-lg flex-grow flex-col justify-between overflow-hidden pb-1"
	>
		<slot />
	</div>
</main>
