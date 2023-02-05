<script lang="ts">
	export let data: any
</script>

<header class="px-2 py-1 w-screen flex flex-row justify-between">
	<div class="flex flex-row gap-1 items-center">
		<img src="/icon.png" alt="GRDN" class="w-4 h-4" />
		<h1 class="font-medium">GRDN</h1>
	</div>
	<div class="flex flex-row items-center transition-all select-none">
		<span>
			{#if data.isHealthy}
				<span
					class="flex flex-row gap-1.5 items-center text-lime-900 bg-lime-100 px-2 py-0.5 rounded-full text-xs font-medium"
				>
					<svg
						viewBox="0 0 100 100"
						xmlns="http://www.w3.org/2000/svg"
						class="fill-lime-500 h-2"
						><circle cx="50" cy="50" r="50" /></svg
					>
					<span>Connected to server</span>
				</span>
			{:else}
				<span
					class="flex flex-row gap-1.5 items-center text-rose-900 bg-rose-100 px-2 py-0.5 rounded-full text-xs font-medium"
				>
					<svg
						viewBox="0 0 100 100"
						xmlns="http://www.w3.org/2000/svg"
						class="fill-rose-500 h-2"
						><circle cx="50" cy="50" r="50" /></svg
					>
					<span>Unable to connect to server</span>
				</span>
			{/if}</span
		>
	</div>
</header>
