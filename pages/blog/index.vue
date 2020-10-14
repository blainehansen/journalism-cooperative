<template lang="pug">

div
	template(v-for="post in posts")
		nuxt-link(:to="post.path")
			h1 {{ post.title }}

</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
	async asyncData({ $content }) {
		const posts = await $content('blog').fetch()
		if (!Array.isArray(posts)) throw new Error()
		return { posts }
	},
})
</script>
