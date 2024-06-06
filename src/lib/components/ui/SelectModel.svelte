<script lang="ts">
	import * as Select from '$lib/components/ui/select'
	import * as Tooltip from '$lib/components/ui/tooltip'
	import { lastModel, openai, openaiKey } from '$lib/openai'
	import { openedDialog } from '$lib/text.store'
	import { type Model } from 'openai/resources/index.mjs'
	import { onMount, tick } from 'svelte'
	import { z } from 'zod'

	const setOpen = async (b: boolean) => {
		b ? openedDialog.set('SELECT-MODEL') : openedDialog.set('')
		const content = document.querySelector('#select-model-trigger')
		await tick()
		if (content instanceof HTMLButtonElement) {
			b ? content.focus() : content.blur()
		}
	}
	$: open = 'SELECT-MODEL' === $openedDialog

	onMount(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.metaKey && event.key === 'g') {
				event.preventDefault()
				setOpen(!open)
			}
		}

		window.addEventListener('keydown', handleKeyDown)
		return () => window.removeEventListener('keydown', handleKeyDown)
	})

	let models: Promise<Model[]> = new Promise(() => {})
	$: {
		models = (async () => {
			if (!$openaiKey) {
				return []
			}

			let retryCount = 0
			while (true) {
				try {
					const response = await openai.models.list()
					return response.data
						.filter((m) => m.id.includes('gpt'))
						.sort((a, b) => b.created - a.created)
				} catch (error) {
					if (++retryCount >= 3) {
						throw error
					}
				}
			}
		})()
	}
</script>

<Tooltip.Root>
	<Tooltip.Trigger>
		<Select.Root
			{open}
			onOpenChange={setOpen}
			onSelectedChange={(e) => lastModel.set({ lastUsedId: z.string().parse(e?.value) })}
		>
			<Select.Trigger
				id="select-model-trigger"
				class="h-auto gap-1 p-1 text-xs hover:bg-muted focus:outline-none"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="12"
					height="12"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="lucide lucide-cpu"
					><rect width="16" height="16" x="4" y="4" rx="2" /><rect
						width="6"
						height="6"
						x="9"
						y="9"
						rx="1"
					/><path d="M15 2v2" /><path d="M15 20v2" /><path d="M2 15h2" /><path d="M2 9h2" /><path
						d="M20 15h2"
					/><path d="M20 9h2" /><path d="M9 2v2" /><path d="M9 20v2" /></svg
				>
				{$lastModel.lastUsedId}
			</Select.Trigger>
			<Select.Content class="max-h-80 overflow-y-auto" sameWidth={false} align="start">
				<Select.Label>Available Models</Select.Label>
				<Select.Separator />
				{#await models}
					<Select.Item disabled value="Loading..."></Select.Item>
				{:then models}
					{#if models.length}
						<div class="flex flex-col">
							{#each models as model}
								<Select.Item
									class="relative inline w-full max-w-60 flex-row place-items-center overflow-hidden text-ellipsis whitespace-nowrap pr-2"
									value={model.id}
									label={model.id}
								></Select.Item>
							{/each}
						</div>
					{:else}
						<Select.Item disabled value="Loading..."></Select.Item>
					{/if}
				{:catch error}
					<Select.Item class="text-red-400" disabled value="An error occurred"></Select.Item>
				{/await}
			</Select.Content>
		</Select.Root>
	</Tooltip.Trigger>
	<Tooltip.Content
		class="z-10 flex flex-row gap-2 bg-muted px-2 text-foreground shadow shadow-background"
	>
		<kbd
			class="inline-flex size-5 items-center justify-center rounded-sm border border-muted bg-background text-sm shadow-kbd"
			>⌘</kbd
		>
		<kbd
			class="inline-flex size-5 items-center justify-center rounded-sm border border-muted bg-background font-mono text-sm shadow-kbd"
			>G</kbd
		>
	</Tooltip.Content>
</Tooltip.Root>
