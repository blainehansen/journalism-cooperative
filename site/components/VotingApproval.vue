<template lang="pug">

.border.rounded.p-2.sm_p-10.spaced
	h2
		span.text-2xl.text-blue-700 {{ approval.position }}
		span.text-md &nbsp;(approval election)

	.content(v-if="overall.winner.length === 1")
		p
			span.font-bold.text-green-600 {{ overall.winner[0] }}
			| &nbsp;is the current winner of this election!
	.content(v-else-if="overall.winner.length > 1")
		p.text-yellow-600 This election is tied between {{ oxfordCommaList(overall.winner) }}.

	.border-t.pt-2(v-for="choice in choicePreferences")
		h3
			span.text-blue-700 {{ choice.choice }}
			span.ml-2
			button.py-1.px-2.text-xs(
				v-for="preference in Preference",
				@click="choice.preference.value = preference",
				:class="{ 'border-blue-700': choice.preference.value.value === preference.value }",
			)
				| {{ preference.display }} ({{ preference.value > 0 ? '+' : '' }}{{ preference.value }})

	.content
		p
			b {{ localWeights }}
			| &nbsp;weights used,&nbsp;
			b {{ remainingWeights }}
			| &nbsp;remaining.
		p(v-if="localVotes === 0") Since you haven't used any weights in this election, your preferences don't count toward the result.
		p(v-else)
			| That number of weights strengthens all your preferences by multiplying them by&nbsp;
			b {{ localVotes.toFixed(2) }}
			| .

	div: button.py-1.px-2.text-xs.font-bold(
		v-for="increment in increments",
		@click="changeWeights(increment)",
		:class="!incrementValid(increment) ? ['cursor-not-allowed', 'text-gray-500'] : []",
	)
		| {{ increment > 0 ? '+' : '' }}{{ increment }}

	h2.mb-1 Rankings:
	.border-t.pt-2(v-for="choice in overall.rankedPreferences")
		h3.text-blue-700 {{ choice.choice }}
		vote-bar(
			:vote="choice.totalVote.value", :min="overall.voteMin", :max="overall.voteMax",
			:local="choice.vote.value.toFixed(2)", :existing="choice.existingVote.toFixed(2)",
		)

	//- then we have the tables. it would be fun to have nested cells? then you can tabulate for each choice?
	//- hr
	//- table.w-full.table-fixed
	//- 	thead: tr
	//- 		th.p-2(:style="{ width: '20%' }")
	//- 		th.p-2 # Voters
	//- 		th.p-2 # Weights
	//- 	tbody
	//- 		tr
	//- 			td.border.p-2.font-bold.border You
	//- 			td.border.p-2.text-center 1
	//- 			td.border.p-2.text-center {{ localWeights }}
	//- 		tr
	//- 			td.border.p-2.font-bold.border Others
	//- 			td.border.p-2.text-center {{ approval.voters }}
	//- 			td.border.p-2.text-center {{ approval.weights }}
	//- 		tr
	//- 			td.bg-gray-100.border.p-2.font-bold Total
	//- 			td.bg-gray-100.border.p-2.text-center {{ approval.voters + 1 }}
	//- 			td.bg-gray-100.border.p-2.text-center {{ approval.weights + localWeights }}


	//- .border-t.pt-4.content(v-for="choice in choicePreferences")
	//- 	table.w-full.table-fixed
	//- 		thead: tr
	//- 			th.p-2(:style="{ width: '20%' }")
	//- 			th.p-2 Total Votes
	//- 		tbody
	//- 			tr
	//- 				td.border.p-2.font-bold You
	//- 				td.border.p-2.text-center {{ choice.vote.value.toFixed(2) }}
	//- 			tr
	//- 				td.border.p-2.font-bold Others
	//- 				td.border.p-2.text-center {{ choice.existingVote.toFixed(2) }}
	//- 			tr
	//- 				td.bg-gray-100.border.p-2.font-bold.border-t Total
	//- 				td.bg-gray-100.border.p-2.text-center {{ choice.totalVote.value.toFixed(2) }}

</template>


<script lang="ts">
import { PropType, defineComponent, ref, computed } from '@vue/composition-api'
import { Approval, Preference, createIncrementChecker, increments, oxfordCommaList } from './voting.utils'

export default defineComponent({
	props: {
		approval: { type: Object as PropType<Approval>, required: true },
		remainingWeights: { type: Number, required: true },
	},
	setup(props, { emit }) {
		const localWeights = ref(0)
		const localVotes = computed(() => Math.sqrt(localWeights.value))
		const incrementValid = createIncrementChecker(() => props.remainingWeights, () => localWeights.value)
		function changeWeights(increment: number) {
			if (!incrementValid(increment)) return
			localWeights.value += increment
			emit('update:remainingWeights', props.remainingWeights - increment)
		}

		const choicePreferences = props.approval.choices.map(({ choice, existingVote }) => {
			const preference = ref(Preference.NEITHER)
			const vote = computed(() => localVotes.value * preference.value.value)
			const totalVote = computed(() => vote.value + existingVote)
			return { choice, preference, vote, totalVote, existingVote  }
		})

		const overall = computed(() => {
			const groups: { [key: number]: string[] } = {}
			let maxVotes = 0
			let voteMin = Infinity
			let voteMax = -Infinity
			const rankedPreferences = []
			for (const choicePreference of choicePreferences) {
				const { choice, totalVote } = choicePreference
				rankedPreferences.push(choicePreference)
				const voteValue = totalVote.value
				const floorVoteValue = Math.max(0, totalVote.value)
				const group = groups[floorVoteValue] || (groups[floorVoteValue] = [])
				group.push(choice)
				maxVotes = Math.max(maxVotes, floorVoteValue)

				voteMin = Math.min(voteMin, voteValue)
				voteMax = Math.max(voteMax, voteValue)
			}
			rankedPreferences.sort((a, b) => b.totalVote.value - a.totalVote.value)

			return { winner: groups[maxVotes], voteMin, voteMax, rankedPreferences }
		})

		return {
			Preference, increments, oxfordCommaList,
			localWeights, localVotes, incrementValid, changeWeights, choicePreferences, overall,
		}
	},
})

</script>
