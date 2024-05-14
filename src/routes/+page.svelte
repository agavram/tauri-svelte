<script>
	import Counter from './Counter.svelte'
	import welcome from '$lib/images/svelte-welcome.webp'
	import welcome_fallback from '$lib/images/svelte-welcome.png'
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="flex flex-col justify-between h-full">
	<h1>hi</h1>
	<input type="text" class="rounded-full mb-4 mx-4 py-1 px-2" />
</div>
