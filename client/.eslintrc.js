/** @type {import('eslint').Linter.Config} */
module.exports = {
    parserOptions: {
        ecmaVersion: 'latest'
    },
    plugins: ['@typescript-eslint'],
    extends: [
        'eslint:recommended',
        'next',
        'next/core-web-vitals',
        'plugin:@typescript-eslint/recommended',
        'prettier'
    ],
    rules: {
        'linebreak-style': ['error', 'unix']
    },
    globals: {
        React: true,
        JSX: true
    },
    ignorePatterns: ['node_modules', '.next', '.vscode']
};
