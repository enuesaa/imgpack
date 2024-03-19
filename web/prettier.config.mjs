export default {
	printWidth: 120,
	tabWidth: 2,
	useTabs: true,
	semi: false,
	singleQuote: true,
	jsxSingleQuote: true,
	endOfLine: 'lf',
	trailingComma: 'none',
	plugins: ['prettier-plugin-svelte'],
	overrides: [
		{
			files: '*.svelte',
			options: { parser: 'svelte' }
		}
	]
}
