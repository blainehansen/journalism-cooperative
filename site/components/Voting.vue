<template lang="pug">

.spaced
	h2.text-3xl You have {{ remainingWeights }} weights left to allocate in all elections.

	voting-approval(
		v-for="approval in approvals", :key="approval.position",
		:approval="approval", :remainingWeights.sync="remainingWeights",
	)

	voting-prioritization(
		v-for="prioritization in prioritizations", :key="prioritization.resource",
		:prioritization="prioritization", :remainingWeights.sync="remainingWeights",
	)

</template>


<script lang="ts">
import { defineComponent, ref } from '@vue/composition-api'
import { maxWeights, randomElections } from './voting.utils'

export default defineComponent({
	setup() {
		const { approvals, prioritizations } = randomElections([
			{ position: "Chief Editor", choices: ["Alice", "Bob", "Cindy", "Darryl"] },
			{ position: "Governance Constitution", choices: ["Version A", "Version B", "Version C", "Version D"] },
			{ position: "Community Funding Percentage", choices: ["%1", "%3", "%5"] },
			{ resource: "Journalism Effort", choices: ["corruption", "politics & society", "science & technology", "business", "culture & arts"] },
			{ resource: "Technical Features", choices: ["single sign-on", "user tag creation", "democratic forum"] },
		])

		const remainingWeights = ref(maxWeights)
		return { approvals, prioritizations, remainingWeights }
	},
})

</script>
