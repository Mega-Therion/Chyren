import { dataAccess } from '@/lib/dal'

export const dynamic = 'force-dynamic'

export async function GET() {
  const statuses = await dataAccess.getStatuses()
  const siteUrl = process.env.NEXT_PUBLIC_SITE_URL || 'https://chyren.io'

  const rss = `<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
<channel>
  <title>CHYREN // Status</title>
  <link>${siteUrl}/status</link>
  <description>Sovereign Event Feed for Chyren</description>
  <language>en-us</language>
  <lastBuildDate>${new Date().toUTCString()}</lastBuildDate>
  <atom:link href="${siteUrl}/status/rss" rel="self" type="application/rss+xml" />
  ${statuses
    .map(
      (status: any) => `
    <item>
      <title>${status.text.slice(0, 50)}${status.text.length > 50 ? '...' : ''}</title>
      <link>${siteUrl}/status#${status.id}</link>
      <guid>${status.id}</guid>
      <pubDate>${new Date(status.created_at).toUTCString()}</pubDate>
      <description><![CDATA[${status.text}]]></description>
    </item>`
    )
    .join('')}
</channel>
</rss>`

  return new Response(rss, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 's-maxage=3600, stale-while-revalidate',
    },
  })
}
