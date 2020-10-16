<template lang="pug">

p hello {{ personName }}

</template>

<script lang="ts">
import { defineComponent } from '@vue/composition-api'

export default defineComponent({
	props: { personName: String },
})
</script>
