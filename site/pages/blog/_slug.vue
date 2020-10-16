<template lang="pug">

.container.mx-auto.p-20.spaced
	article.content
		h1 {{ post.title }}
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
