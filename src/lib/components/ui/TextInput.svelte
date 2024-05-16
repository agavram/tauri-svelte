<script lang="ts">
	import { onMount, tick } from 'svelte'
	import { pending } from '$lib/text.store.js'

	let message = ''
	let input: HTMLSpanElement
	let savedRange: Range | undefined
	let loadingResponse = false

	pending.subscribe((value) => {
		loadingResponse = !!value
	})

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault()
			handleSubmit()
			input.focus()
		}
	}

	function handleSubmit() {
		if (loadingResponse) {
			return
		}

		if (message.trim()) {
			pending.set(message.trim())
			message = ''
		}
	}

	onMount(() => {
		console.log('mounted')
		const handleSlash = (event: KeyboardEvent) => {
			if (event.key === '/' && event.target === document.body) {
				input.focus()
				event.preventDefault()
			}
		}
		const handleFocus = () => {
			console.log('grabbed focus')
			if (!input.focus) {
				return
			}

			if (savedRange) {
				let selection = window.getSelection()
				selection?.removeAllRanges()
				selection?.addRange(savedRange)
				return
			}

			let range = document.createRange()
			let selection = window.getSelection()

			range.selectNodeContents(input)
			range.collapse(false)

			selection?.removeAllRanges()
			selection?.addRange(range)
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

<form on:submit|preventDefault={handleSubmit} class="relative z-10 flex flex-row px-4">
	<span
		class="block max-h-44 min-h-3 w-full overflow-y-scroll rounded-md border border-input bg-transparent px-3 py-2 pr-10 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
		role="textbox"
		bind:this={input}
		tabindex="0"
		placeholder={!message || message === '\n' ? 'Send a message' : ''}
		on:keydown={handleKeyDown}
		contenteditable="plaintext-only"
		bind:innerText={message}
	>
	</span>
	{#if loadingResponse}
		<svg
			width="24"
			height="24"
			viewBox="0 0 24 24"
			xmlns="http://www.w3.org/2000/svg"
			class="absolute bottom-[8px] right-6 fill-foreground"
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
		<button on:click={handleSubmit}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="feather feather-arrow-up-circle absolute bottom-[8px] right-6 stroke-foreground"
				><circle cx="12" cy="12" r="10"></circle><polyline points="16 12 12 8 8 12"></polyline><line
					x1="12"
					y1="16"
					x2="12"
					y2="8"
				></line></svg
			>
		</button>
	{/if}
</form>
