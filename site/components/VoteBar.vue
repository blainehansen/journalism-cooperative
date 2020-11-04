<template lang="pug">


.relative.bg-gray-100.h-6.flex.items-stretch.w-full.text-xs.mb-6.mt-6
	.whitespace-no-wrap.overflow-visible.flex.items-center.absolute.inset-y-0(
		:style="position", :class="vote < 0 ? ['bg-red-200'] : ['bg-green-200', 'justify-end']",
	)
		span.mx-2
			| {{ vote.toFixed(2) }}
			template(v-if="local !== '0.00'") &nbsp;({{ local }} + {{ existing }})

	template(v-if="max > 0 && min < 0")
		.absolute.bg-gray-400.line.mid(:style="{ left: position.zeroPoint }")
		span.absolute.just-under.mid(:style="{ left: position.zeroPoint }") 0

	.absolute.bg-gray-400.line.left-0
	span.absolute.just-over.left-0 {{ min.toFixed(2) }}

	.absolute.bg-gray-400.line.right-0
	span.absolute.just-over.right-0.text-right {{ max.toFixed(2) }}

</template>


<script lang="ts">
import { PropType, defineComponent, ref, computed, Ref, ComputedRef } from '@vue/composition-api'

function votePosition(vote: number, min: number, max: number) {
	const clampedAbsMin = Math.abs(Math.min(min, 0))
	const totalWidth = clampedAbsMin + Math.abs(Math.max(max, 0))
	const absMin = Math.abs(min)
	// const absMax = Math.abs(max)
	const absVote = Math.abs(vote)

	const zeroPoint = clampedAbsMin / totalWidth
	const width = absVote / totalWidth
	const left = vote < 0 ? (absMin - absVote) / totalWidth : zeroPoint
	const prct = (n: number) => `${n * 100}%`
	return { width: prct(width), left: prct(left), zeroPoint: prct(zeroPoint) }
}

export default defineComponent({
	props: {
		vote: { type: Number, required: true },
		local: { type: String, required: true },
		existing: { type: String, required: true },
		min: { type: Number, required: true },
		max: { type: Number, required: true },
	},
	setup(props) {
		const position = computed(() => {
			return votePosition(props.vote, props.min, props.max)
		})
		return { position }
	}
})

</script>

<style scoped>

.line {
	width: 1px;
	top: -10%;
	bottom: -10%;
}

.mid {
	transform: translateX(-50%);
}

.just-under {
	top: 120%;
}

.just-over {
	bottom: 120%;
}

</style>
