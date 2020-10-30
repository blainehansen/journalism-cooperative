<template lang="pug">

.border.rounded.p-10.spaced
	h2.text-2xl This is an election to prioritize {{ prioritization.resource }}

	.content: p
		b The current ranking:
		template(v-for="choice in sortedChoices")
			br
			| {{ choice }}

	div(v-for="choice in choicePreferences")
		h1.text-2xl {{ choice.choice }}
		.content
			p
				| You've used&nbsp;
				b {{ choice.weights.value }}
				| &nbsp;weights on this choice, with&nbsp;
				b {{ remainingWeights }}
				| &nbsp;remaining.
			p(v-if="choice.votes.value === 0") Since you haven't used any weights on this choice, your preferences don't count toward the result.
			p(v-else)
				| That number of weights strengthens your preference by multiplying it by&nbsp;
				b {{ choice.votes.value }}
				| .

			br
			p
				b {{ choice.existingVoters }}
				| &nbsp;other voters have used a total of&nbsp;
				b {{ choice.existingWeights }}
				| &nbsp;weights, giving a total of&nbsp;
				b {{ choice.existingVote }}
				| &nbsp;votes, all leading to an overall total of&nbsp;
				b {{ choice.totalVote.value }}.

		p Give your preference for this choice.
		button.control(
			v-for="preference in Preference",
			@click="choice.preference.value = preference",
			:class="{ 'text-blue-700': choice.preference.value.value === preference.value }",
		)
			| {{ preference.display }} ({{ preference.value > 0 ? '+' : '' }}{{ preference.value }})

		p Use the buttons below to increase or decrease how many weights you're using in this election.
		div: button.control(
			v-for="increment in increments",
			@click="choice.changeWeights(increment)",
			:class="!choice.incrementValid(increment) ? ['cursor-not-allowed', 'text-gray-500'] : []",
		)
			| {{ increment > 0 ? '+' : '' }}{{ increment }}

</template>


<script lang="ts">
import { PropType, defineComponent, ref, computed, Ref, ComputedRef } from '@vue/composition-api'
import { Prioritization, Preference, createIncrementChecker, increments, oxfordCommaList } from './voting.utils'

export default defineComponent({
	props: {
		prioritization: { type: Object as PropType<Prioritization>, required: true },
		remainingWeights: { type: Number, required: true },
	},
	setup(props, { emit }) {
		const choicePreferences = props.prioritization.choices.map(({ choice, existingVoters, existingWeights, existingVote }) => {
			const weights = ref(0)
			const votes = computed(() => Math.sqrt(weights.value))
			const preference = ref(Preference.NEITHER)
			const vote = computed(() => votes.value * preference.value.value)
			const totalWeights = computed(() => weights.value + existingWeights)
			const totalVote = computed(() => vote.value + existingVote)

			const incrementValid = createIncrementChecker(() => props.remainingWeights, () => weights.value)
			function changeWeights(increment: number) {
				if (!incrementValid(increment)) return
				weights.value += increment
				emit('update:remainingWeights', props.remainingWeights - increment)
			}

			return {
				choice, weights, votes, preference, vote, totalWeights, totalVote,
				existingVoters, existingWeights, existingVote,
				incrementValid, changeWeights,
			}
		})

		const sortedChoices = computed(() => {
			return choicePreferences
				.slice()
				.sort((a, b) => b.totalVote.value - a.totalVote.value)
				.map(choice => `${choice.choice} (weights: ${choice.totalWeights.value}, vote: ${choice.totalVote.value})`)
		})

		return { increments, choicePreferences, sortedChoices, Preference }
	},
})

</script>
