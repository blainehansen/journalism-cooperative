<template lang="pug">

.container.mx-auto.spaced
	h1.text-6xl Proof of Concept

	h2.text-4xl As you can see, I'm no designer!

	.content
		p I hope you decide to join me in this project!
		p Here's a basic email form that will send you a verification email without using any newsletter management services such as mailchimp, but from a server I wrote and deployed.
		p Controlling our own email system would be quite important for minimizing costs and giving our members control.

	client-only(placeholder="loading")
		email-signup

		hr
		.content
			h2.text-3xl Voting Mechanism Demo
			p This area is a very simple demo of how a quadratic voting and a combination of quadratic and approval voting work. The user interface is extremely unintuitive, but it's just a little toy I threw together to get a feel for the mechanisms.
			p I'll briefly explain what's happening, but sometime soon I'll release a blog post explaining more fully, and arguing for the merits of these forms of voting.
			p There are five fake elections, three "approval" elections where some position must be filled with a single winner, and two "prioritization" elections where multiple choices are merely being ranked.
			p
				| The principles of&nbsp;
				a(href="https://towardsdatascience.com/what-is-quadratic-voting-4f81805d5a06") Quadratic Voting
				| &nbsp;come into play with the limited supply of "weights" can be used across all elections. More weights can be used to increase the strength of your preferences, but as more and more are used, they get less and less effective. This allows voters to express how deeply they care about each issue in a mathematically balanced way.
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

			p Enjoy!

		voting

</template>
