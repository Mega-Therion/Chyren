import { NextResponse } from 'next/server';

const QDRANT_URL = process.env.QDRANT_URL ?? 'http://localhost:6333';
const COLLECTION = 'knowledge_matrix';

export async function GET() {
  try {
    // 1. Fetch collection info
    const infoResp = await fetch(`${QDRANT_URL}/collections/${COLLECTION}`);
    const info = await infoResp.json();

    // 2. Scroll through points to get a sample for visualization
    // In a real system, we'd use a projected 2D map, but for a sample, we can just grab 100 points.
    const scrollResp = await fetch(`${QDRANT_URL}/collections/${COLLECTION}/points/scroll`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        limit: 200,
        with_payload: true,
        with_vector: false,
      }),
    });
    const scroll = await scrollResp.json();

    return NextResponse.json({
      count: info.result?.points_count ?? 0,
      status: info.result?.status ?? 'unknown',
      points: scroll.result?.points ?? [],
    });
  } catch (error) {
    console.error('Observability API Error:', error);
    return NextResponse.json({ error: 'Qdrant unreachable' }, { status: 500 });
  }
}
