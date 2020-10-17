<template lang="pug">

.content(v-if="state.successful")
	p.text-green-600 Success! You've been signed up for the newsletter.

.content(v-else, @keyup.enter="send")
	div
		div: input.border.rounded.p-4(v-model="state.email", placeholder="you@example.com")
		p(v-if="state.loading") loading...
		p.text-red-600(v-else-if="state.touched && error") {{ error }}
	div: button.border.rounded.p-4(@click="send") Send Verification Email

</template>


<script lang="ts">
import http from '@/plugins/axios'
import { AxiosError } from 'axios'
import { defineComponent, reactive, computed } from '@vue/composition-api'

function validateEmail(email: string) {
	return email !== '' && email.match(/^.+@.+\..+$/)
}
const invalidMessage = "That email isn't valid!"

export default defineComponent({
	setup() {
		const state = reactive({
			email: '',
			touched: false,
			loading: false,
			successful: false,
			requestErrorMessage: null as null | string,
		})

		const emailInvalid = computed(() => !validateEmail(state.email))
		const emailInvalidMessage = computed(() => emailInvalid.value ? invalidMessage : null)
		const error = computed(() => state.requestErrorMessage || emailInvalidMessage.value)

		async function send() {
			state.requestErrorMessage = null
			if (!state.email.trim()) return
			state.touched = true
			if (emailInvalid.value) return

			state.loading = true
			await http.post('/subscribe', { email: state.email })
				.then(() => {
					state.successful = true
				})
				.catch(error => {
					state.successful = false
					if (error.response && error.response.status === 400) {
						state.requestErrorMessage = invalidMessage
						return
					}
					state.requestErrorMessage = "There was some unknown error. Please let me know so I can fix it."
				})

			state.loading = false
		}

		return { state, emailInvalid, emailInvalidMessage, error, send }
	},
})

</script>
