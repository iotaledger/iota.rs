module.exports = {
    env: {
        node: true,
        browser: true,
        commonjs: true,
        es2021: true,
    },
    plugins: ['@typescript-eslint'],
    extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended'],
    parser: '@typescript-eslint/parser',
    parserOptions: {
        ecmaVersion: 12,
        sourceType: 'module',
    },
    rules: {
        '@typescript-eslint/ban-ts-comment': [
            'error',
            { 'ts-ignore': 'allow-with-description' },
        ],
    },
};
