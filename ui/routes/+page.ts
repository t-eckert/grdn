import type { PageLoad } from "./$types"

const HOST = "http://localhost:8000"

export const load: PageLoad = async ({ fetch }) => {
	const healthCheck = await fetch(`${HOST}/api/health`)
	const health = await healthCheck.text()
	const isHealthy = health.includes("OK")

	return {
		weather: {},
		isHealthy,
	}
}
