<template lang="pug">

.container.mx-auto.p-20.content
	h1.text-4xl Verify Email

	client-only(placeholder="loading")
		p(v-if="state === VerifyState.LOADING")
			| verifying email...
		p.text-green-600(v-if="state === VerifyState.SUCCESS")
			| Your email has been verified!
		template(v-if="state === VerifyState.NOT_FOUND")
			p.text-red-600
				| The link you used is missing the secret verification token.
			p.text-red-600
				| Double check the link to make sure it exactly matches what was originally sent to you.
		p.text-red-600(v-if="state === VerifyState.UNKNOWN")
			| There was some unknown error. Please let me know so I can fix it.

</template>

<script lang="ts">
import http from '@/plugins/axios'
import { defineComponent, ref } from '@vue/composition-api'

enum VerifyState { LOADING, SUCCESS, NOT_FOUND, UNKNOWN }

export default defineComponent({
	setup(_, { root }) {
		const verificationToken = root.$route.query.t
		const state = ref(!!verificationToken ? VerifyState.LOADING : VerifyState.NOT_FOUND)

		if (!!verificationToken)
			http.post('/verify', { verification_token: verificationToken })
				.then(() => {
					state.value = VerifyState.SUCCESS
				})
				.catch(error => {
					state.value = VerifyState.UNKNOWN
				})

		return { state, VerifyState }
	}
})

</script>
