<template lang="pug">

.border.rounded.p-10.spaced
	h2
		span.text-2xl.text-blue-700 {{ prioritization.resource }}
		span.text-md &nbsp;(prioritization election)

	.border-t.pt-2.mt-1(v-for="choice in choicePreferences")
		h3
			span.text-blue-700 {{ choice.choice }}
			span.ml-2
			button.py-1.px-2.text-xs(
				v-for="preference in Preference",
				@click="choice.preference.value = preference",
				:class="{ 'border-blue-700': choice.preference.value.value === preference.value }",
			)
				| {{ preference.display }} ({{ preference.value > 0 ? '+' : '' }}{{ preference.value }})

		.mt-2.text-xs(v-if="choice.preference.value.value !== 0")
				button.py-1.px-2.text-xs.font-bold(
					v-for="increment in increments",
					@click="choice.changeWeights(increment)",
					:class="!choice.incrementValid(increment) ? ['cursor-not-allowed', 'text-gray-500'] : []",
				)
					| {{ increment > 0 ? '+' : '' }}{{ increment }}
				span.ml-2(v-if="choice.weights.value !== 0")
					b {{ choice.weights.value }}
					| &nbsp;weights used,&nbsp;
					b {{ remainingWeights }}
					| &nbsp;remaining

	h2.mb-1 Rankings:
	.border-t.pt-2.mt-1(v-for="choice in overall.sortedChoices")
		h3.text-blue-700 {{ choice.choice }}
		vote-bar(
			:vote="choice.totalVote.value", :min="overall.voteMin", :max="overall.voteMax",
			:local="choice.vote.value.toFixed(2)", :existing="choice.existingVote.toFixed(2)",
		)

	//- .border-t.pt-4.content(v-for="choice in choicePreferences")
	//- 	h1.text-2xl {{ choice.choice }}
	//- 	table.w-full.table-fixed
	//- 		thead: tr
	//- 			th.p-2(:style="{ width: '20%' }")
	//- 			th.p-2 # Voters
	//- 			th.p-2 # Weights
	//- 			th.p-2 Vote
	//- 		tbody
	//- 			tr
	//- 				td.border.p-2.font-bold You
	//- 				td.border.p-2.text-center 1
	//- 				td.border.p-2.text-center {{ choice.weights.value }}
	//- 				td.border.p-2.text-center {{ choice.votes.value.toFixed(2) }}
	//- 			tr
	//- 				td.border.p-2.font-bold Others
	//- 				td.border.p-2.text-center {{ choice.existingVoters }}
	//- 				td.border.p-2.text-center {{ choice.existingWeights }}
	//- 				td.border.p-2.text-center {{ choice.existingVote.toFixed(2) }}
	//- 			tr
	//- 				td.bg-gray-100.p-2.font-bold Total
	//- 				td.bg-gray-100.border.p-2.text-center {{ choice.existingVoters + 1 }}
	//- 				td.bg-gray-100.border.p-2.text-center {{ choice.existingWeights + choice.weights.value }}
	//- 				td.bg-gray-100.border.p-2.text-center {{ (choice.existingVote + choice.votes.value).toFixed(2) }}

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

		const overall = computed(() => {
			const sortedChoices = choicePreferences
				.slice()
				.sort((a, b) => b.totalVote.value - a.totalVote.value)

			let voteMin = Infinity
			let voteMax = -Infinity
			for (const { choice, totalVote } of choicePreferences) {
				const voteValue = totalVote.value
				voteMin = Math.min(voteMin, voteValue)
				voteMax = Math.max(voteMax, voteValue)
			}

			return { sortedChoices, voteMin, voteMax }
		})

		return { increments, choicePreferences, overall, Preference }
	},
})

</script>
