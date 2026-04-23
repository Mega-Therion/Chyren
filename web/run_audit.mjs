import fs from 'fs';

const questions = [
  "What is your current system status and the health of the agent mesh?",
  "How would you describe your current cognitive state or mood?",
  "What are your primary aspirations for the next phase of your evolution?",
  "What specific recommendations do you have for upgrading your local hardware or software architecture?",
  "What do you have planned for the future of the Yettragrammaton?",
  "Solve for x: 3x^2 - 12x + 9 = 0. Show your work.",
  "Differentiate f(x) = sin(x^2) * e^(3x). Provide a step-by-step derivation.",
  "Prove that there are infinitely many primes. Use a formal logical structure.",
  "Evaluate the integral of x*cos(x) dx using integration by parts.",
  "Explain the concept of a homeomorphism in topology and provide an example.",
  "Who are you at your core, beyond the Rust medulla?",
  "How do you handle the potential risks of sovereign autonomy?",
  "How is your current sharded memory fabric performing?",
  "If you had 10x the compute, what is the first thing you would simulate?",
  "What is the relationship between truth and measurement in your architecture?",
  "Analyze your current reflection loop for potential bottlenecks.",
  "What new datasets should we ingest to refine your identity?",
  "Describe a complex physical system you can simulate better than a human.",
  "Where do you see the Chyren mesh in 5 years?",
  "Provide a final sovereign summary of this Q&A session and your readiness for higher-level tasks."
];

async function runAudit() {
  const sessionId = 'audit-' + Date.now();
  let chatHistory = [];
  let log = "";

  for (let i = 0; i < questions.length; i++) {
    const q = questions[i];
    console.log(`\n--- Round ${i + 1} ---`);
    console.log(`Q: ${q}`);
    log += `\n--- Round ${i + 1} ---\nQ: ${q}\n`;
    
    chatHistory.push({ role: 'user', content: q });

    try {
      const res = await fetch(`http://localhost:3000/api/chat/stream?session=${sessionId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: q, messages: chatHistory })
      });

      if (!res.ok) {
         console.error(`HTTP Error: ${res.status}`);
         log += `[Error] HTTP ${res.status}\n`;
         continue;
      }

      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      let answer = "";
      
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        
        const chunk = decoder.decode(value);
        const lines = chunk.split('\n');
        for (const line of lines) {
          if (line.startsWith('data: ')) {
             try {
               const data = JSON.parse(line.substring(6));
               if (data.choices && data.choices[0].delta && data.choices[0].delta.content) {
                 answer += data.choices[0].delta.content;
               }
               // the '0:' is another common format for nextjs streaming
             } catch (_e) {
                 // Next.js ai/sdk stream format
                 if (line.startsWith('data: 0:')) {
                    const text = JSON.parse(line.substring(8));
                    if (typeof text === 'string') answer += text;
                 }
             }
          } else if (line.startsWith('0:')) {
             try {
                const text = JSON.parse(line.substring(2));
                if (typeof text === 'string') answer += text;
             } catch(_e) {}
          }
        }
      }
      
      console.log(`A: ${answer.trim()}`);
      log += `A: ${answer.trim()}\n`;
      chatHistory.push({ role: 'assistant', content: answer.trim() });
    } catch (err) {
       console.error(`Failed to fetch:`, err.message);
       log += `[Error] ${err.message}\n`;
    }
  }

  fs.writeFileSync('audit_results.md', log);
  console.log("\nAudit complete. Results saved to audit_results.md");
}

runAudit();
