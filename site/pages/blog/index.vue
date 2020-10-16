<template lang="pug">

.container.mx-auto.p-20.spaced.content
	nuxt-link(v-for="post in posts", :to="post.path", :key="post.path")
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
