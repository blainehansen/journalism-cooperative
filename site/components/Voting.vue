<template lang="pug">

.spaced
	.content
		h2.text-3xl Voting Mechanism Demo
		p This area is a very simple demo of how quadratic voting and a combination of quadratic and approval voting work. The user interface is extremely unintuitive, but it's just a little toy I threw together to get a feel for the mechanisms.
		p I'll briefly explain what's happening, but sometime soon I'll release a blog post explaining more fully, and arguing for the merits of these forms of voting.
		p There are five fake elections: three "approval" elections where some position must be filled with a single winner; and two "prioritization" elections where multiple choices are merely being ranked.
		p
			| The principles of&nbsp;
			a(href="https://towardsdatascience.com/what-is-quadratic-voting-4f81805d5a06") Quadratic Voting
			| &nbsp;come into play with the limited supply of "weights" can be used across all elections. More weights can be used to increase the strength of your preferences, but as more and more are used, they get less and less effective. This allows voters to express how deeply they care about each issue but in a mathematically balanced way.
		p
			| The two different types of elections use these weights in different ways:
			ul
				li.mb-2
					p
						| In the approval elections, voters use their weights on&nbsp;
						i the entire election
						| &nbsp;rather than a single choice. This means that the fair dynamics of&nbsp;
						a(href="https://electionscience.org/approval-voting-101/") Approval Voting
						| &nbsp;(which allows voters to express their preferences accurately and honestly) are still in play, but voters can make their approvals more intense in some elections, representing the fact that they care differently about the outcomes.
					p I've included a "disapprove" setting as well, partially as an experiment.
				li: p In the prioritization elections, weights are simply used on the choices themselves. These elections aren't "winner take all", so they don't need the balancing of the approval mechanism.

		p To help you get a feel for what it would be like to vote with other people also voting, every time you refresh this page a new set of "voters" with random preferences is generated, and included in the vote counts along with your votes.

		p Enjoy!

	hr
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
			{ position: "Director of Reporting", choices: ["Lois Lane", "Peter Parker", "Christine Everhart", "Clark Kent"] },
			{ position: "Director of Technology", choices: ["Tony Stark", "Bruce Banner", "Justin Hammer", "Lucius Fox"] },
			{ position: "Director of Operations", choices: ["Pepper Potts", "Nick Fury", "Phil Coulson"] },
			{ resource: "Journalism Effort", choices: ["social accountability", "politics", "science & technology", "business", "culture & arts"] },
			{ resource: "Technical Features", choices: ["single sign-on", "user tag creation", "democratic forum"] },
		])

		const remainingWeights = ref(maxWeights)
		return { approvals, prioritizations, remainingWeights }
	},
})

</script>
