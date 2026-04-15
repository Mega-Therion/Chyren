import { NextResponse } from 'next/server';

export async function GET() {
  try {
    const response = await fetch('http://localhost:9090/metrics', {
      next: { revalidate: 0 } // Disable caching for real-time metrics
    });
    
    if (!response.ok) {
      return NextResponse.json({ error: 'Failed to fetch metrics from Medulla' }, { status: 500 });
    }

    const data = await response.text();
    
    // Simple parser for Prometheus text format to JSON
    const metrics: Record<string, number> = {};
    const lines = data.split('\n');
    for (const line of lines) {
      if (line && !line.startsWith('#')) {
        const [key, val] = line.split(' ');
        if (key && val) {
          metrics[key] = parseFloat(val);
        }
      }
    }

    return NextResponse.json({ 
      timestamp: new Date().toISOString(),
      metrics 
    });
  } catch (error) {
    console.error('Metrics proxy error:', error);
    return NextResponse.json({ error: 'Medulla metrics server unreachable' }, { status: 502 });
  }
}
