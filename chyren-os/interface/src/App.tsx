import { useState, useEffect } from 'react';
import './chyren.css';

export default function App() {
  const [status, setStatus] = useState<'nominal' | 'alert'>('nominal');
  const [messages] = useState([{ text: "Chyren is online. Ready to orchestrate.", sender: 'chyren' }]);

  // Simulate ADCCL drift
  useEffect(() => {
    const timer = setInterval(() => {
      setStatus(Math.random() > 0.9 ? 'alert' : 'nominal');
    }, 5000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="min-h-screen bg-[#0a0a0a] text-[#e2e8f0] flex flex-col font-mono relative overflow-hidden">
      <div className="chyren-scanlines" />
      
      {/* Integrity HUD */}
      <header className="p-4 flex items-center justify-between border-b border-[#00f2ff]/30 glass-panel">
        <div className="flex items-center gap-3">
          <div className={`w-4 h-4 rounded-full ${status === 'nominal' ? 'bg-[#00f2ff] shadow-[0_0_10px_#00f2ff]' : 'bg-[#ff00ff] shadow-[0_0_10px_#ff00ff]'} animate-pulse`} />
          <h1 className="text-xl font-bold tracking-widest text-[#00f2ff] neon-glow">CHYREN</h1>
        </div>
      </header>
      
      <div className="flex flex-1 overflow-hidden">
        {/* Ledger Sidebar */}
        <aside className="w-64 p-4 glass-panel border-r border-[#00f2ff]/20 hidden md:block">
          <h2 className="text-[#00f2ff] mb-4 text-sm uppercase">Master Ledger</h2>
          <div className="text-[10px] space-y-2 opacity-70">
            <div>[0x4A..] COMMITTED</div>
            <div>[0x9F..] ADCCL: 0.98</div>
            <div>[0x1C..] PROVIDER: ANTHROPIC</div>
          </div>
        </aside>

        <main className="flex-1 p-4 space-y-4">
          {messages.map((m, i) => (
            <div key={i} className="p-4 rounded border border-[#00f2ff]/20 glass-panel">
              {m.text}
            </div>
          ))}
        </main>
      </div>

      <footer className="p-4 border-t border-[#00f2ff]/30 glass-panel">
        <div className="flex gap-2">
          <input className="flex-1 bg-transparent border border-[#00f2ff]/30 p-2 rounded focus:outline-none focus:border-[#ff00ff]" placeholder="Command..." />
        </div>
      </footer>
    </div>
  );
}
