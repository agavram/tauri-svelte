<script lang="ts">
	import * as Select from '$lib/components/ui/select'
	import { lastModel, openai } from '$lib/openai'
	import { openedDialog } from '$lib/text.store'
	import { onMount } from 'svelte'
	import { z } from 'zod'

	const setOpen = (b: boolean) => {
		b ? openedDialog.set('SELECT-MODEL') : openedDialog.set('')
	}
	$: open = 'SELECT-MODEL' === $openedDialog

	onMount(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.metaKey && event.key === 'g') {
				setOpen(true)
			}
		}

		window.addEventListener('keydown', handleKeyDown)
		return () => window.removeEventListener('keydown', handleKeyDown)
	})

	let models = (async () => {
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
</script>

<Select.Root
	{open}
	onOpenChange={setOpen}
	onSelectedChange={(e) => lastModel.set({ lastUsedId: z.string().parse(e?.value) })}
>
	<Select.Trigger class="h-auto gap-1 p-1 text-xs hover:bg-muted focus:outline-none">
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
		{#await models}
			<Select.Item disabled value="Loading..."></Select.Item>
		{:then models}
			<Select.Label>Available Models</Select.Label>
			<Select.Separator />
			<div class="flex flex-col">
				{#each models as model}
					<Select.Item
						class="inline-block w-full flex-row place-items-center overflow-hidden text-ellipsis text-nowrap"
						value={model.id}
						label={model.id}
					></Select.Item>
				{/each}
			</div>
		{:catch error}
			<Select.Item class="text-red-400" disabled value="An error occurred"></Select.Item>
		{/await}
	</Select.Content>
</Select.Root>
