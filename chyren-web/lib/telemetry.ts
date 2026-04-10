import { trace } from '@opentelemetry/api';

export async function withSpan<T>(name: string, fn: (span: any) => Promise<T>): Promise<T> {
  const tracer = trace.getTracer('chyren-web');
  return await tracer.startActiveSpan(name, async (span) => {
    try {
      const result = await fn(span);
      span.setStatus({ code: 1 }); // OK
      return result;
    } catch (err: any) {
      span.recordException(err);
      span.setStatus({ code: 2, message: err.message }); // ERROR
      throw err;
    } finally {
      span.end();
    }
  });
}
