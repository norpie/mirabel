<script lang="ts">
	import showdown from 'showdown';

	let { markdown = $bindable() }: { markdown: string } = $props();

	import { mode } from 'mode-watcher';

	const lightModeClassMap = new Map<string, string>([
		// ["* + *", "mt-0 mb-4"],
		['p', 'break-words leading-normal text-gray-900'],
		['h1', 'leading-tight border-b text-4xl font-semibold mb-4 mt-6 pb-2'],
		['h2', 'leading-tight border-b text-2xl font-semibold mb-4 mt-6 pb-2'],
		['h3', 'leading-snug text-lg font-semibold mb-4 mt-6'],
		['h4', 'leading-none text-base font-semibold mb-4 mt-6'],
		['h5', 'leading-tight text-sm font-semibold mb-4 mt-6'],
		['h6', 'leading-tight text-sm font-semibold text-gray-600 mb-4 mt-6'],
		['blockquote', 'text-base border-l-4 border-gray-300 pl-4 pr-4 text-gray-600'],
		['code', 'font-mono text-sm inline bg-gray-200 rounded px-1 py-05'],
		['pre', 'bg-gray-100 rounded p-4'],
		['pre code', 'block bg-transparent p-0 overflow-visible rounded-none'],
		['ul', 'text-base pl-8 list-disc'],
		['ol', 'text-base pl-8 list-decimal'],
		[
			'kbd',
			'text-xs inline-block rounded border px-1 py-05 align-middle font-normal font-mono shadow'
		],
		['table', 'text-base border-gray-600'],
		['th', 'border py-1 px-3'],
		['td', 'border py-1 px-3'],
		['.highlight pre', 'bg-gray-100 !important']
	]);

	const darkModeClassMap = new Map<string, string>([
		['p', 'break-words leading-normal text-gray-200'],
		[
			'h1',
			'leading-tight border-b text-4xl font-semibold mb-4 mt-6 pb-2 text-white border-gray-700'
		],
		[
			'h2',
			'leading-tight border-b text-2xl font-semibold mb-4 mt-6 pb-2 text-white border-gray-700'
		],
		['h3', 'leading-snug text-lg font-semibold mb-4 mt-6 text-white'],
		['h4', 'leading-none text-base font-semibold mb-4 mt-6 text-white'],
		['h5', 'leading-tight text-sm font-semibold mb-4 mt-6 text-gray-300'],
		['h6', 'leading-tight text-sm font-semibold text-gray-400 mb-4 mt-6'],
		['blockquote', 'text-base border-l-4 border-gray-600 pl-4 pr-4 text-gray-400'],
		['code', 'font-mono text-sm inline bg-zinc-700 text-gray-200 rounded px-1 py-05'],
		['pre', 'bg-gray-800 rounded p-4 text-gray-200'],
		['pre code', 'block bg-transparent p-0 overflow-visible rounded-none text-gray-200'],
		['ul', 'text-base pl-8 list-disc text-gray-200'],
		['ol', 'text-base pl-8 list-decimal text-gray-200'],
		[
			'kbd',
			'text-xs inline-block rounded border border-gray-600 px-1 py-05 align-middle font-normal font-mono shadow text-gray-200'
		],
		['table', 'text-base border-gray-600 text-gray-200'],
		['th', 'border border-gray-600 py-1 px-3 text-gray-200'],
		['td', 'border border-gray-600 py-1 px-3 text-gray-200'],
		['.highlight pre', 'bg-gray-900 !important']
	]);

	let bindings = new Map<string, string>([]);

	let conv = new showdown.Converter();

	let converted = $state('');

	$effect(() => {
		if ($mode == 'light') {
			bindings = lightModeClassMap.keys().map((key) => ({
				type: 'output',
				regex: new RegExp(`(?:<${key}>)|(?:<${key} (.*?)>)`, 'g'),
				replace: `<${key} class="${lightModeClassMap.get(key)}" $1>`
			}));
		} else {
			bindings = darkModeClassMap.keys().map((key) => ({
				type: 'output',
				regex: new RegExp(`(?:<${key}>)|(?:<${key} (.*?)>)`, 'g'),
				replace: `<${key} class="${darkModeClassMap.get(key)}" $1>`
			}));
		}
		conv = new showdown.Converter({
			extensions: [...bindings]
		});
		converted = conv.makeHtml(markdown);
	});
</script>

<div class="markdown-holder break-words leading-normal text-gray-900">
	{@html converted}
</div>

<style>
/* Remove the top padding of the first h1-h6 children */
:global(.markdown-holder > h1:first-child),
:global(.markdown-holder > h2:first-child),
:global(.markdown-holder > h3:first-child),
:global(.markdown-holder > h4:first-child),
:global(.markdown-holder > h5:first-child),
:global(.markdown-holder > h6:first-child) {
    margin-top: 0;
}

</style>
