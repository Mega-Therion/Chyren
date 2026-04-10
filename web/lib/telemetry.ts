import { trace, type Span } from '@opentelemetry/api';

export async function withSpan<T>(name: string, fn: (span: Span) => Promise<T>): Promise<T> {
  const tracer = trace.getTracer('chyren-web');
  return await tracer.startActiveSpan(name, async (span) => {
    try {
      const result = await fn(span);
      span.setStatus({ code: 1 }); // OK
      return result;
    } catch (err: unknown) {
      const e = err instanceof Error ? err : new Error(String(err));
      span.recordException(e);
      span.setStatus({ code: 2, message: e.message }); // ERROR
      throw err;
    } finally {
      span.end();
    }
  });
}
