import { z } from 'zod';

/**
 * Validated environment schema.
 * Uses safeParse so the build never crashes from missing optional vars.
 */
const envSchema = z.object({
  NEXT_PUBLIC_API_BASE_URL: z.string().optional().default(''),
  NEON_DATABASE_URL: z.string().optional(),
  FIREBASE_PROJECT_ID: z.string().optional(),
  FIREBASE_CLIENT_EMAIL: z.string().optional(),
  FIREBASE_PRIVATE_KEY: z.string().optional(),
  VERCEL_URL: z.string().optional(),
  NODE_ENV: z.enum(['development', 'test', 'production']).optional().default('development'),
});

const parsed = envSchema.safeParse(process.env);

if (!parsed.success) {
  console.warn('⚠ Environment validation warnings:', parsed.error.flatten().fieldErrors);
}

export const env = parsed.success ? parsed.data : envSchema.parse({});
