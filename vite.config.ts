import { sveltekit } from "@sveltejs/kit/vite"
import type { UserConfig } from "vite"

const config: UserConfig = {
	plugins: [sveltekit()],
	test: {
		include: ["ui/**/*.{test,spec}.{js,ts}"],
	},
	server: {
		fs: {
			allow: ["./"],
		},
	},
}

export default config
