import adapter from "@sveltejs/adapter-auto"
import { vitePreprocess } from "@sveltejs/kit/vite"

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		files: {
			routes: "ui/routes",
			appTemplate: "ui/app.html",
			hooks: {
				client: "ui/hooks.client",
				server: "ui/hooks.server",
			},
			lib: "ui/lib",
			serviceWorker: "ui/service-worker",
			errorTemplate: "ui/error.html",
		},
	},
}

export default config
