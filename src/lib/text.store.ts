import { writable } from 'svelte/store'

function pendingMessage() {
	const { subscribe, set } = writable<string>('')

	return {
		subscribe,
		set
	}
}

export const pending = pendingMessage()
