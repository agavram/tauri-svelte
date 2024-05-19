<script lang="ts">
	import { Button } from '$lib/components/ui/button'
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu'
	import { lastModel, openai } from '$lib/openai'

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

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button
			variant="ghost"
			class="flex h-auto flex-row justify-center gap-1 px-2 py-1 text-xs"
			builders={[builder]}
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
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="max-h-80 overflow-y-scroll">
		<DropdownMenu.Group>
			{#await models}
				<DropdownMenu.Item>Loading...</DropdownMenu.Item>
			{:then models}
				<DropdownMenu.Label>Available Models</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<div class="flex flex-col">
					{#each models as model}
						<DropdownMenu.Trigger>
							<Button
								variant="ghost"
								class="w-full justify-start"
								on:click={() => lastModel.set({ lastUsedId: model.id })}
							>
								{model.id}
							</Button>
						</DropdownMenu.Trigger>
					{/each}
				</div>
			{:catch error}
				<DropdownMenu.Item class="text-red-400">An error occurred</DropdownMenu.Item>
			{/await}
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
