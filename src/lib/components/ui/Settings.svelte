<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog'
	import { Input } from '$lib/components/ui/input'
	import { Label } from '$lib/components/ui/label'
	import * as Tooltip from '$lib/components/ui/tooltip'
	import { openedDialog } from '$lib/text.store'
	import { onMount, tick } from 'svelte'
	import { defaults, superForm, type Infer, numberProxy } from 'sveltekit-superforms'
	import { zod } from 'sveltekit-superforms/adapters'
	import { openAiSchema, openaiKey, openaiTemp } from '../../openai'
	import Button from './button/button.svelte'
	import { get } from 'svelte/store'
	import { cn } from '../../utils'
	import { fade } from 'svelte/transition'

	let errored = false
	let submitted = false

	const {
		form,
		errors,
		enhance,
		reset: rawReset
	} = superForm<Infer<typeof openAiSchema>, { status: number; text: string }>(
		defaults(zod(openAiSchema)),
		{
			SPA: true,
			resetForm: false,
			clearOnSubmit: 'none',
			validators: zod(openAiSchema),
			onUpdated(event) {
				if (event.form.valid && !submitted) {
					submitted = true
					setTimeout(() => {
						submitted = false
					}, 1500)
				} else if (!event.form.valid && !errored) {
					errored = true
					setTimeout(() => {
						errored = false
					}, 1000)
				}

				openaiKey.set(event.form.data.apiKey)
				openaiTemp.set(event.form.data.temperature)
			},
			validationMethod: 'oninput'
		}
	)
	const reset = () => {
		rawReset({
			data: {
				apiKey: get(openaiKey),
				temperature: get(openaiTemp)
			}
		})
	}
	reset()

	const setOpen = async (b: boolean) => {
		b ? openedDialog.set('OPEN-SETTINGS') : openedDialog.set('')
		const content = document.querySelector('#open-settings-trigger')
		await tick()
		if (!(content instanceof HTMLButtonElement)) {
			return
		}
		if (b) {
			content.focus()
		} else {
			content.blur()
			reset()
		}
	}
	$: open = 'OPEN-SETTINGS' === $openedDialog

	onMount(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.metaKey && event.key === ',') {
				event.preventDefault()
				setOpen(!open)
			}
		}

		window.addEventListener('keydown', handleKeyDown)
		return () => window.removeEventListener('keydown', handleKeyDown)
	})

	let temperatureProxy = numberProxy(form, 'temperature')
</script>

<Dialog.Root {open} onOpenChange={setOpen}>
	<Tooltip.Root>
		<Dialog.Trigger>
			<Tooltip.Trigger>
				<Button
					variant="ghost"
					class="h-auto gap-1 border border-input p-1 text-xs hover:bg-muted hover:text-foreground focus:outline-none"
					id="open-settings-trigger"
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
						class="lucide lucide-settings"
						><path
							d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
						/><circle cx="12" cy="12" r="3" /></svg
					>
					Settings
				</Button>
			</Tooltip.Trigger>
		</Dialog.Trigger>
		<Tooltip.Content
			class="z-10 flex flex-row gap-2 bg-muted px-2 text-foreground shadow shadow-background"
		>
			<kbd
				class="inline-flex size-5 items-center justify-center rounded-sm border border-muted bg-background text-sm shadow-kbd"
				>⌘</kbd
			>
			<kbd
				class="inline-flex size-5 items-center justify-center rounded-sm border border-muted bg-background font-mono text-sm shadow-kbd"
				>,</kbd
			>
		</Tooltip.Content>
	</Tooltip.Root>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header class="place-items-start">
			<Dialog.Title>Settings</Dialog.Title>
			<Dialog.Description>
				Make edits to your OpenAI API Key and model temperature
			</Dialog.Description>
		</Dialog.Header>
		<form method="POST" use:enhance>
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-x-4 gap-y-2">
					<Label for="apiKey" class="text-left">OpenAI API Key</Label>
					<Input name="apiKey" class="col-span-3" type="password" bind:value={$form.apiKey} />
					<Label for="apiKey" class="col-span-full text-xs text-secondary"
						><a
							class="inline-flex flex-row place-items-center gap-1"
							rel="noopener noreferrer"
							target="_blank"
							href="https://platform.openai.com/api-keys"
							>https://platform.openai.com/api-keys
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
								class="lucide lucide-external-link"
								><path d="M15 3h6v6" /><path d="M10 14 21 3" /><path
									d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"
								/></svg
							></a
						></Label
					>
				</div>
				<div class="grid grid-cols-4 items-center gap-x-4 gap-y-2">
					<Label for="temperature" class="text-left">Temperature</Label>
					<Input
						name="temperature"
						class="col-span-3"
						type="number"
						step="any"
						bind:value={$temperatureProxy}
					/>
					{#if $errors.temperature}
						<Label for="temperature" class="col-span-full text-xs text-destructive-foreground">
							{$errors.temperature}
						</Label>
					{/if}
					<Label for="temperature" class="col-span-full text-xs text-muted-foreground">
						The sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the
						output more random, while lower values like 0.2 will make it more focused and
						deterministic.
					</Label>
				</div>
			</div>
			<Dialog.Footer>
				<Button name="submit" class={cn('relative w-32 place-self-end', errored && 'animate-shake')} type="submit">
					{#if submitted}
						<svg
							class="absolute"
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							in:fade><path d="M20 6 9 17l-5-5" /></svg
						>
					{:else}
						<span class="absolute" in:fade>Save changes</span>
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
