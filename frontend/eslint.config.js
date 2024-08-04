import js from '@eslint/js';
import simpleImportSort from 'eslint-plugin-simple-import-sort';
import vue from 'eslint-plugin-vue';
import ts from 'typescript-eslint';

export default [
  {
    languageOptions: {
      ecmaVersion: 'latest',
    },
  },

  // js
  js.configs.recommended,

  // ts
  ...ts.configs.recommended,
  {
    rules: {
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-expressions': ['error', { allowTernary: true }],
    },
  },

  // vue
  ...vue.configs['flat/recommended'],
  {
    // files: ['*.vue', '**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: ts.parser,
      },
    },
  },
  {
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/no-unused-vars': ['error', { ignorePattern: '^_' }],
      'vue/max-attributes-per-line': ['error', { singleline: 5 }],
    },
  },

  // Sort imports
  {
    plugins: {
      'simple-import-sort': simpleImportSort,
    },
    rules: {
      'simple-import-sort/imports': [
        'error',
        {
          groups: [
            [
              '^\\u0000', // all side effects (0 at start)
              '^[^/\\.].*\u0000$', // external types (0 at end)
              '^\\..*\u0000$', // internal types (0 at end)
              '^@?\\w', // Starts with @
              '^[^.]', // any
              '^\\.', // local
            ],
          ],
        },
      ],
      'simple-import-sort/exports': 'error',
    },
  },
];
