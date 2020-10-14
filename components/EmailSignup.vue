<template lang="pug">

div(v-if="successful")
	p.text-green-600 Success! You've been signed up for the newsletter.

.content(v-else, @keyup.enter="send")
	div
		div: input.border.rounded.p-4(v-model="email", placeholder="you@example.com")
		p.text-red-600(v-if="touched && error") {{ error }}
	div: button.border.rounded.p-4(@click="send") Send Verification Email

</template>


<script lang="ts">
// import axios from 'axios'
// import { MicroChimpClient, MicroChimpResult } from 'micro-chimp'
// const client = new MicroChimpClient(axios)

import { defineComponent, ref, computed } from '@vue/composition-api'
function validateEmail(email: string) {
	return email !== '' && email.match(/^.+@.+\..+$/)
}

export default defineComponent({
	setup() {
		const email = ref('')
		const touched = ref(false)
		const successful = ref(false)
		const requestErrorMessage = ref(null as null | string)

		const emailInvalid = computed(() => !validateEmail(email.value))
		const emailInvalidMessage = computed(() => emailInvalid.value ? "That email isn't valid!" : null)
		const error = computed(() => requestErrorMessage.value || emailInvalidMessage.value)

		function send() {
			if (!email.value.trim()) return
			touched.value = true
			if (emailInvalid.value) return

			// requestErrorMessage.value = null
			// const result = await client.new_email(email.value)
			console.log('signing up', email.value)
			successful.value = true
		}

		return { email, touched, successful, requestErrorMessage, emailInvalid, emailInvalidMessage, error, send }
	},
})

</script>
