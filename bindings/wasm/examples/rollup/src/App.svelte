<script>
	import * as client from 'iota-rs-wasm/web'
	let promise;
	
	promise = client.addNode('https://nodes.comnet.thetangle.org').then(res => {
		promise = client.getNodeInfo().then(res => {
			setTimeout(() => {
				promise = client
					.getNewAddress('RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA')
					.index(1)
					.security(3)
					.generate()
			}, 3000)
			return res
		})
		return res
	})

	$: promise = promise.then(res => JSON.stringify(res))
</script>

<main>
	{#await promise}
		<h1>...waiting</h1>
	{:then res}
		<h1>Promise resolved</h1>
		<span>{ res }</span>
	{:catch error}
		<h1 style="color: red">Promise rejected: {error}</h1>
	{/await}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>