async function testApi() {
  const endpoint = 'https://chyren-web.vercel.app/api/chat/stream?session=test-123';
  
  console.log('Sending test message to production API...');
  
  try {
    const res = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        message: "Are you fully operational?",
        messages: [{ role: 'user', content: "Are you fully operational?" }]
      })
    });
    
    console.log(`Status: ${res.status}`);
    
    if (res.ok) {
      const text = await res.text();
      console.log('Response streams back ok:');
      console.log(text);
    } else {
      console.log(`Error body: ${await res.text()}`);
    }
  } catch (e) {
    console.error('Fetch failed:', e);
  }
}

testApi();
