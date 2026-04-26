/**
 * Structured logger for Chyren Web.
 * Uses pino in Node.js, falls back to console in edge/browser.
 */

type LogFn = (msg: string, context?: Record<string, unknown>) => void;

interface Logger {
  info: LogFn;
  warn: LogFn;
  error: (msg: string, err: unknown, context?: Record<string, unknown>) => void;
}

function createLogger(): Logger {
  try {
    // Dynamic import avoids bundling issues in edge runtime
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const pino = require('pino');
    const isProduction = process.env.NODE_ENV === 'production';
    const instance = pino({
      level: process.env.LOG_LEVEL || (isProduction ? 'info' : 'debug'),
      base: { component: 'chyren-web' },
    });
    return {
      info: (msg, ctx) => instance.info(ctx, msg),
      warn: (msg, ctx) => instance.warn(ctx, msg),
      error: (msg, err, ctx) =>
        instance.error(
          { err: err instanceof Error ? { message: err.message, stack: err.stack } : err, ...ctx },
          msg,
        ),
    };
  } catch {
    // Fallback to console if pino is unavailable (edge runtime, browser)
    return {
      // eslint-disable-next-line no-console
      info: (msg, ctx) => console.log(`[INFO] ${msg}`, ctx || ''),
      warn: (msg, ctx) => console.warn(`[WARN] ${msg}`, ctx || ''),
      error: (msg, err, ctx) => console.error(`[ERROR] ${msg}`, err, ctx || ''),
    };
  }
}

export const logger = createLogger();
export const logInfo = logger.info;
export const logError = logger.error;
