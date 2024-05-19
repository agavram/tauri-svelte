<script lang="ts">
	import { onMount, tick } from 'svelte'

	export let onSubmit: (s: string) => void
	export let pending: boolean

	export let message = ''
	let input: HTMLSpanElement
	let savedRange: Range | undefined

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
		const handleSlash = (event: KeyboardEvent) => {
			if (event.key === '/' && event.target === document.body) {
				input.focus()
				event.preventDefault()
			}
		}
		const handleFocus = async () => {
			if (!input.focus) {
				return
			}

			input.focus()
			if (savedRange) {
				let selection = window.getSelection()

				selection?.removeAllRanges()
				selection?.addRange(savedRange)
			} else {
				let range = document.createRange()
				let selection = window.getSelection()

				range.selectNodeContents(input)
				range.collapse(false)

				selection?.removeAllRanges()
				selection?.addRange(range)
			}
		}
		const getRange = () => {
			let selection = window.getSelection()
			if (selection?.focusNode?.parentNode?.contains(input)) {
				savedRange = selection.getRangeAt(0)
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
			window.removeEventListener('keydown', handleSlash)
			window.removeEventListener('focus', handleFocus)
			window.removeEventListener('blur', getRange)
		}
	})
</script>

<form on:submit|preventDefault={handleSubmit} class="relative w-full">
	<span
		class="block max-h-44 min-h-3 w-full overflow-y-scroll rounded-md border border-input bg-transparent px-3 py-2 pr-10 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
		role="textbox"
		bind:this={input}
		tabindex="0"
		on:keydown={handleKeyDown}
		contenteditable="plaintext-only"
		bind:innerText={message}
	>
	</span>
	{#if !message || message === '\n'}
		<span
			class="pointer-events-none absolute left-[13px] top-1/2 -translate-y-1/2 transform text-sm text-muted-foreground"
			>Send a message</span
		>
	{/if}
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
