import { FlatCompat } from '@eslint/eslintrc'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const compat = new FlatCompat({
  baseDirectory: __dirname,
  languageOptions: {
    parserOptions: {
      project: './tsconfig.json',
      tsconfigRootDir: __dirname,
    },
  },
})

const config = [
  ...compat.extends('next/core-web-vitals', 'next/typescript'),
  {
    ignores: ['.next', '.vercel', 'node_modules', 'next-env.d.ts', 'eslint.config.mjs'],
  },
  {
    files: ['scripts/hash_helper.js'],
    rules: {
      // Utility script; allow CJS-style require without tripping repo-wide lint rules.
      '@typescript-eslint/no-require-imports': 'off',
    },
  },
  {
    files: ['**/*.ts', '**/*.tsx'],
    languageOptions: {
      parserOptions: {
        project: './tsconfig.json',
        tsconfigRootDir: __dirname,
      },
    },
    rules: {
      // Prevent accidental console logs left in production code
      'no-console': ['warn', { allow: ['warn', 'error'] }],

      // Catch unused variables (allow underscore-prefixed intentional ignores)
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
      ],

      // Disallow explicit `any` type usage
      '@typescript-eslint/no-explicit-any': 'warn',

      // Enforce consistent type imports
      '@typescript-eslint/consistent-type-imports': [
        'error',
        { prefer: 'type-imports', fixStyle: 'inline-type-imports' },
      ],

      // Require explicit return types on module-level functions
      '@typescript-eslint/explicit-module-boundary-types': 'off',

      // Prevent floating promises (critical in async API routes)
      '@typescript-eslint/no-floating-promises': 'error',

      // Enforce React hooks rules
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',

      // Enforce no-dangerously-set-inner-html
      'react/no-danger': 'error',
    },
  },
]

export default config
