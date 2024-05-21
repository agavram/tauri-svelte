<script lang="ts">
	import autosize from 'autosize'
	import { onMount, tick } from 'svelte'

	export let onSubmit: (s: string) => void
	export let pending: boolean

	let message = ''
	let input: HTMLTextAreaElement
	let savedRange: [number, number] | undefined

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault()
			handleSubmit()
			input.focus()
		}
	}

	function handleSubmit() {
		if (pending) {
			return
		}

		if (message.trim()) {
			onSubmit(message.trim())
			message = ''
		}
	}

	onMount(() => {
		autosize(input)
		const handleSlash = (event: KeyboardEvent) => {
			if (event.key === '/' || (event.key === 'l' && event.metaKey)) {
				input.focus()
				event.preventDefault()
			}
		}
		const handleFocus = async () => {
			if (!input.focus) {
				return
			}

			if (savedRange) {
				input.setSelectionRange(...savedRange)
			} else {
				input.setSelectionRange(input.value.length, input.value.length)
			}
		}

		const getRange = () => {
			if (document.activeElement === input) {
				savedRange = [input.selectionStart, input.selectionEnd]
			} else {
				savedRange = undefined
			}
		}

		window.addEventListener('keydown', handleSlash)
		window.addEventListener('focus', handleFocus)
		window.addEventListener('blur', getRange)
		;(async () => {
			await tick()
			input.focus()
		})()

		return () => {
			autosize.destroy(input)
			window.removeEventListener('keydown', handleSlash)
			window.removeEventListener('focus', handleFocus)
			window.removeEventListener('blur', getRange)
		}
	})
</script>

<form on:submit|preventDefault={handleSubmit} class="relative w-full">
	<textarea
		class="block max-h-44 min-h-3 w-full overflow-y-scroll rounded-md border border-input bg-transparent px-3 py-2 pr-10 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
		bind:this={input}
		rows="1"
		on:keydown={handleKeyDown}
		bind:value={message}
		placeholder="Send a message"
	></textarea>
	{#if !!pending}
		<svg
			width="24"
			height="24"
			viewBox="0 0 24 24"
			xmlns="http://www.w3.org/2000/svg"
			class="absolute bottom-[8px] right-2 fill-foreground"
			><style>
				.spinner_P7sC {
					transform-origin: center;
					animation: spinner_svv2 0.75s infinite linear;
				}
				@keyframes spinner_svv2 {
					100% {
						transform: rotate(360deg);
					}
				}
			</style><path
				fill="inherit"
				d="M10.14,1.16a11,11,0,0,0-9,8.92A1.59,1.59,0,0,0,2.46,12,1.52,1.52,0,0,0,4.11,10.7a8,8,0,0,1,6.66-6.61A1.42,1.42,0,0,0,12,2.69h0A1.57,1.57,0,0,0,10.14,1.16Z"
				class="spinner_P7sC"
			/></svg
		>
	{:else}
		<button on:click={handleSubmit} class="absolute bottom-2 right-2">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="lucide lucide-send"><path d="m22 2-7 20-4-9-9-4Z" /><path d="M22 2 11 13" /></svg
			>
		</button>
	{/if}
</form>
