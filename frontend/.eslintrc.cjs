/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution')

module.exports = {
    root: true,
    'extends': [
        'plugin:vue/vue3-recommended',
        'eslint:recommended',
        'plugin:promise/recommended',
        '@vue/eslint-config-typescript',
        '@vue/eslint-config-prettier/skip-formatting'
    ],
    plugins: ['simple-import-sort'],
    parserOptions: {
        ecmaVersion: 'latest'
    },
    rules: {
        'vue/multi-word-component-names': 'off',
        'vue/no-unused-vars': ['error', {ignorePattern: '^_'}],
        'no-unused-vars': 'off',
        '@typescript-eslint/no-unused-vars': 'error',
        'simple-import-sort/imports': 'error',
        'simple-import-sort/exports': 'error',
    },
}
