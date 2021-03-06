import * as util from 'util'
import { NuxtConfig } from '@nuxt/types'
import tailwindTypography from '@tailwindcss/typography'


const config: NuxtConfig = {
	head: {
		title: 'Journalism Cooperative',
		meta: [
			{ charset: 'utf-8' },
			{ name: 'viewport', content: 'width=device-width, initial-scale=1' },
			{ hid: 'description', name: 'description', content: 'Journalism Cooperative' }
		]
	},

	env: {
		API_URL: process.env.NODE_ENV === 'production'
			? process.env.API_URL || (() => { throw new Error("API_URL isn't set in production build") })()
			: 'http://localhost:5050',
	},

	target: 'static',
	generate: {
		fallback: false,
	},

	modules: ['@nuxt/content'],
	// https://composition-api.nuxtjs.org/
	buildModules: ['@nuxt/typescript-build', '@nuxtjs/tailwindcss'],
	plugins: ['@/plugins/index.ts'],
	server: {
		port: 8080,
	},
	components: true,

	// https://composition-api.nuxtjs.org/
	content: {
		liveEdit: false,
	},

	// https://storybook.nuxtjs.org/options
	storybook: { typescript: { check: true } },

	// https://tailwindcss.com/docs/using-with-preprocessors
	// https://tailwindcss.nuxtjs.org/tailwind-config
	tailwindcss: {
		cssPath: '~/styles/main.css',
		config: {
			separator: '_',
			important: true,
			theme: {
				container: {
					center: true,
					padding: '3rem',
				},
			},

			plugins: [
				tailwindTypography,
			],

			// https://type-scale.com/

			purge: {
				enabled: process.env.NODE_ENV === 'production',
				content: ['components/**/*.vue', 'layouts/**/*.vue', 'pages/**/*.vue', 'plugins/**/*.ts', 'nuxt.config.ts'],
				options: {
					whitelist: ['h1', 'h2', 'h3', 'p', 'blockquote', 'strong' /* etc. */],
				},
			},

			future: {
				removeDeprecatedGapUtilities: true,
				purgeLayersByDefault: true,
				defaultLineHeights: true,
				standardFontWeights: true,
			},
		},
	},
}

export default config
