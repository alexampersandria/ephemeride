import prettier from 'eslint-config-prettier'
import eslint from '@eslint/js'
import tsEslint from 'typescript-eslint'
import svelteEslint from 'eslint-plugin-svelte'
import svelteParser from 'svelte-eslint-parser'
import globals from 'globals'
import svelteConfig from './svelte.config'

import { includeIgnoreFile } from '@eslint/compat'
import { fileURLToPath } from 'node:url'
const gitignorePath = fileURLToPath(new URL('../.gitignore', import.meta.url))

/** @type {import('eslint').Linter.Config[]} */
export default [
  includeIgnoreFile(gitignorePath),
  { ignores: ['eslint.config.ts'] },
  prettier,
  eslint.configs.recommended,
  ...tsEslint.configs.recommended,
  ...svelteEslint.configs['flat/prettier'],
  // jsdoc.configs['flat/recommended'],
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parser: svelteParser,
      parserOptions: { svelteConfig, parser: tsEslint.parser },
      globals: { ...globals.browser },
    },
  },
  {
    rules: {
      'css-unused-selector': 'off',
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': [
        'warn', // or "error"
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_',
        },
      ],
    },
  },
]
