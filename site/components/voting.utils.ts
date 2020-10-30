export type NonEmpty<T> = [T, ...T[]]
export type NonLone<T> = [T, T, ...T[]]

export function oxfordCommaList(items: NonLone<string>) {
	return `${items.slice(0, -1).join(", ")}, and ${items[items.length - 1]}`
}

export function createIncrementChecker(remainingWeights: () => number, localWeights: () => number) {
	return function(increment: number) {
		if (increment > 0) return (remainingWeights() - increment) >= 0
		else return (localWeights() + increment) >= 0
	}
}

export const Preference = {
	APPROVE: { display: "Approve", value: 1 },
	NEITHER: { display: "Neither", value: 0 },
	DISAPPROVE: { display: "Disapprove", value: -1 },
} as const
export type Preference = (typeof Preference)[keyof typeof Preference]

export const maxWeights = 100
export const increments = [-10, -5, -2, -1, 1, 2, 5, 10]

function randomInt(min: number, max: number) {
	return Math.floor((Math.random() * (max - min + 1)) + min)
}
function randomPreference(): Preference {
	const r = Math.random()
	if (r >= 0.7) return Preference.APPROVE
	else if (r < 0.3) return Preference.DISAPPROVE
	else return Preference.NEITHER
}

type Voter = { weights: number }
function createElectorate(n: number): Voter[] {
	return Array.from({ length: n }, () => ({ weights: maxWeights }))
}
function voterSubset(voters: Voter[], electionCount: number) {
	const subset: Voter[] = []
	for (const voter of voters) {
		if (Math.random() > 0.7) continue

		const weights = randomInt(0, voter.weights / electionCount)
		if (weights === 0) continue

		voter.weights -= weights
		subset.push({ weights })
	}

	return subset
}


export type RawApproval = { position: string, choices: NonLone<string> }
export type Approval = { position: string, voters: number, weights: number, choices: NonLone<{ choice: string, existingVote: number }> }
export type RawPrioritization = { resource: string, choices: NonLone<string> }
export type Prioritization = { resource: string, choices: NonLone<{ choice: string, existingVoters: number, existingWeights: number, existingVote: number }> }

function randomPrioritization(voters: Voter[], { resource, choices }: RawPrioritization): Prioritization {
	const choicesCount = choices.length
	const finalChoices = choices.map(choice => ({ choice, existingVoters: 0, existingWeights: 0, existingVote: 0 })) as Prioritization['choices']
	for (const choice of finalChoices) {
		for (const voter of voters) {
			const preference = randomPreference()
			if (preference.value === Preference.NEITHER.value) continue
			const weights = randomInt(0, voter.weights / choicesCount)
			if (weights === 0) continue

			voter.weights -= weights
			choice.existingWeights += weights
			choice.existingVote += Math.sqrt(weights) * preference.value
			choice.existingVoters++
		}
	}

	return { resource, choices: finalChoices }
}

function randomApproval(voters: Voter[], { position, choices }: RawApproval) {
	const finalChoices = choices.map(choice => ({ choice, existingVote: 0 })) as Approval['choices']
	let weights = 0
	let votersCount = 0
	for (const voter of voters) {
		votersCount++
		weights += voter.weights
		for (const choice of finalChoices) {
			const preference = randomPreference()
			if (preference.value === Preference.NEITHER.value) continue

			choice.existingVote += Math.sqrt(voter.weights) * preference.value
		}
	}

	return { position, weights, voters: votersCount, choices: finalChoices }
}

export function randomElections(elections: (RawApproval | RawPrioritization)[]) {
	const electorate = createElectorate(40)
	let totalElections = elections.length

	const approvals: Approval[] = []
	const prioritizations: Prioritization[] = []
	for (const election of elections) {
		if ('position' in election)
			approvals.push(randomApproval(voterSubset(electorate, totalElections / 4), election))
		else
			prioritizations.push(randomPrioritization(voterSubset(electorate, totalElections / 4), election))
	}

	return { approvals, prioritizations }
}

// import * as util from 'util'
// console.log(util.inspect(randomElections(
// 	[
// 		{ position: "Chief Editor", choices: ["Alice", "Bob", "Cindy", "Darryl"] },
// 		{ position: "Governance Constitution", choices: ["Version A", "Version B", "Version C", "Version D"] },
// 		{ position: "Community Funding Percentage", choices: ["%1", "%3", "%5"] },
// 		{ resource: "Journalism Effort", choices: ["corruption", "politics & society", "science & technology", "business", "culture & arts"] },
// 		{ resource: "Technical Features", choices: ["single sign-on", "user tag creation", "democratic forum"] },
// 	],
// ), { depth: null, colors: true }))
