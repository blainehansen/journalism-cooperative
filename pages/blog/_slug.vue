<template lang="pug">
article
	nuxt-content(:document="post")

</template>

<script lang="ts">
// https://content.nuxtjs.org/
import Vue from 'vue'

export default Vue.extend({
	async asyncData({ $content, params }) {
		const post = await $content('blog', params.slug).fetch()
		// console.log(post.dir)
		return { post }
	},
})
</script>
