<template lang="pug">

.border.rounded.p-10.spaced
	h2.text-2xl This is an approval election to decide the {{ approval.position }}

	.content(v-if="winningGroup.length === 1")
		p.text-green-600: b {{ winningGroup[0] }} is the current winner of this election!
	.content(v-else-if="winningGroup.length > 1")
		p.text-yellow-600 This election is tied between {{ oxfordCommaList(winningGroup) }}.

	.content
		p
			| You've used&nbsp;
			b {{ localWeights }}
			| &nbsp;weights in this election, with&nbsp;
			b {{ remainingWeights }}
			| &nbsp;remaining.
		p(v-if="localVotes === 0") Since you haven't used any weights in this election, your preferences don't count toward the result.
		p(v-else)
			| That number of weights strengthens all your preferences by multiplying them by&nbsp;
			b {{ localVotes }}
			| .
		p Use the buttons below to increase or decrease how many weights you're using in this election.

		br
		p
			b {{ approval.voters }}
			| &nbsp;other voters have given preferences in this election, with a total of&nbsp;
			b {{ approval.weights }}
			| &nbsp;weights.


	div: button.control(
		v-for="increment in increments",
		@click="changeWeights(increment)",
		:class="!incrementValid(increment) ? ['cursor-not-allowed', 'text-gray-500'] : []",
	)
		| {{ increment > 0 ? '+' : '' }}{{ increment }}

	div(v-for="choice in choicePreferences")
		h1
			span.text-2xl {{ choice.choice }}&nbsp;
			span.text-lg ({{ choice.totalVote.value }})
		p Give your preference for this choice.
		button.control(
			v-for="preference in Preference",
			@click="choice.preference.value = preference",
			:class="{ 'text-blue-700': choice.preference.value.value === preference.value }",
		)
			| {{ preference.display }} ({{ preference.value > 0 ? '+' : '' }}{{ preference.value }})

		p
			| Your total vote for this choice is&nbsp;
			b {{ choice.vote.value }}
			| &nbsp;and other voters had a total of&nbsp;
			b {{ choice.existingVote }}
			| &nbsp;leading to an overall total of&nbsp;
			b {{ choice.totalVote.value }}.

</template>


<script lang="ts">
import { PropType, defineComponent, ref, computed, Ref, ComputedRef } from '@vue/composition-api'
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

		const winningGroup = computed(() => {
			const groups: { [key: number]: string[] } = {}
			let maxVotes = 0
			for (const { choice, totalVote } of choicePreferences) {
				const voteValue = Math.max(0, totalVote.value)
				const group = groups[voteValue] || (groups[voteValue] = [])
				group.push(choice)
				maxVotes = Math.max(maxVotes, voteValue)
			}

			return groups[maxVotes]
		})

		return { Preference, increments, oxfordCommaList, localWeights, localVotes, incrementValid, changeWeights, choicePreferences, winningGroup }
	},
})

</script>
