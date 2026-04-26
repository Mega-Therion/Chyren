const fetch = require('node-fetch');

async function testSupabase() {
  const url = process.env.SUPABASE_URL;
  const key = process.env.SUPABASE_SERVICE_KEY;
  
  if (!url || !key) {
    console.log('Missing SUPABASE_URL or SUPABASE_SERVICE_KEY');
    return;
  }

  const base = url.replace(/\/$/, '');
  const headers = {
    apikey: key,
    Authorization: `Bearer ${key}`,
  };

  try {
    console.log(`Testing Supabase at ${base}...`);
    const resp = await fetch(`${base}/rest/v1/family_profiles?select=count`, { headers });
    console.log(`Status: ${resp.status} ${resp.statusText}`);
    const body = await resp.text();
    console.log(`Body: ${body.slice(0, 100)}`);
  } catch (err) {
    console.error('Fetch failed:', err.message);
  }
}

testSupabase();
