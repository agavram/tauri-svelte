<script lang="ts">
	import 'highlight.js/styles/base16/ros-pine.min.css'
	import '../app.css'
	import { getCurrent } from '@tauri-apps/api/window'
</script>

<main class="flex h-screen flex-col rounded-md border-2 border-zinc-900">
	<button
		class="min-h-5 w-full cursor-default"
		on:mousedown={() => {
			getCurrent().startDragging()
		}}
	></button>
	<div
		class="mx-auto flex h-full w-full max-w-screen-lg flex-grow flex-col justify-between overflow-hidden pb-1"
	>
		<slot />
	</div>
</main>
