// test_ari_ws.js
const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:3000/api/ari');

ws.on('open', () => {
  console.log('WebSocket opened');
  // send a silent PCM chunk (24kHz, 16‑bit mono, 0.1s)
  const sampleRate = 24000;
  const durationSec = 0.1;
  const samples = sampleRate * durationSec;
  const buffer = Buffer.alloc(samples * 2, 0); // silence
  const b64 = buffer.toString('base64');
  ws.send(JSON.stringify({ type: 'user-audio', data: b64 }));
  console.log('Sent silent audio');
});

ws.on('message', (data) => {
  const msg = JSON.parse(data);
  if (msg.type === 'assistant-audio') {
    console.log('Received audio chunk (base64 length)', msg.data.length);
  } else {
    console.log('Received unknown message', msg);
  }
});

ws.on('error', (err) => {
  console.error('WebSocket error', err);
});

ws.on('close', () => {
  console.log('WebSocket closed');
});
