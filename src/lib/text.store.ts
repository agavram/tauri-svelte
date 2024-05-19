import Dexie, { type Table } from 'dexie'
import { writable } from 'svelte/store'

export type Message = {
	role: 'assistant' | 'user'
	content: string
}

export type DexieMessage = {
	id: number
	cid: number
} & Message

export type DexieChat = {
	id: number
	title: string
}

export const chatHistory = new Dexie('chat-history') as Dexie & {
	messages: Table<DexieMessage>
	chats: Table<DexieChat>
}

chatHistory.version(1).stores({
	messages: '++id, cid',
	chats: 'id'
})

export const selectedConversation = writable<number>(new Date().getTime())
