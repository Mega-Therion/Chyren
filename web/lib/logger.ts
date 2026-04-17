import pino from 'pino';

const isProduction = process.env.NODE_ENV === 'production';

export const logger = pino({
  level: process.env.LOG_LEVEL || (isProduction ? 'info' : 'debug'),
  transport: isProduction
    ? undefined
    : {
        target: 'pino-pretty',
        options: {
          colorize: true,
        },
      },
  base: {
    env: process.env.NODE_ENV,
    component: 'chyren-web',
  },
});

export const logError = (msg: string, err: unknown, context?: Record<string, unknown>) => {
  logger.error(
    {
      err: err instanceof Error ? { message: err.message, stack: err.stack } : err,
      ...context,
    },
    msg
  );
};

export const logInfo = (msg: string, context?: Record<string, unknown>) => {
  logger.info(context, msg);
};
