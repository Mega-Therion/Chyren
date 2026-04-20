'use client';

import React, { useEffect, useState } from 'react';
import { AnimatePresence } from 'framer-motion';
import { Eye, Database, Activity, Zap } from 'lucide-react';
import { AvogadroMatrix } from '@/components/AvogadroMatrix';

interface Point {
  id: string | number;
  payload: {
    name: string;
    realm: string;
    description: string;
  };
}

interface Stats {
  count: number;
  status: string;
  points: Point[];
}

export default function ObservabilityPage() {
  const [stats, setStats] = useState<Stats | null>(null);

  useEffect(() => {
    fetch('/api/observability/stats')
      .then((res) => res.json())
      .then((data) => setStats(data))
      .catch((err) => console.error(err));
  }, []);

  return (
    <div className="min-h-screen bg-black text-white p-8 font-sans selection:bg-blue-500/30">
      <div className="max-w-6xl mx-auto space-y-8">
        
        {/* Header */}
        <div className="flex items-center justify-between border-b border-white/10 pb-6">
          <div className="flex items-center gap-4">
            <div className="p-3 bg-blue-500/10 rounded-2xl border border-blue-500/20">
              <Eye className="w-8 h-8 text-blue-400" />
            </div>
            <div>
              <h1 className="text-3xl font-bold tracking-tight">The Eye</h1>
              <p className="text-gray-400">Sovereign Knowledge Matrix Observability</p>
            </div>
          </div>
          <div className="flex gap-4">
             <StatCard icon={<Database className="w-4 h-4" />} label="Entities" value={stats?.count?.toLocaleString() ?? '...'} />
             <StatCard icon={<Activity className="w-4 h-4" />} label="Status" value={stats?.status ?? '...'} color="text-green-400" />
          </div>
        </div>

        {/* Main Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          
          {/* Visualizer Panel */}
          <div className="lg:col-span-2 relative h-[500px] group">
            <AvogadroMatrix />
            {!stats && (
              <div className="absolute inset-0 flex items-center justify-center bg-black/40 backdrop-blur-sm rounded-3xl">
                <div className="flex flex-col items-center gap-4">
                  <Zap className="w-12 h-12 text-cyan-400 animate-pulse" />
                  <p className="text-cyan-400 font-medium tracking-widest uppercase text-xs">Synchronizing Resonance...</p>
                </div>
              </div>
            )}
          </div>

          {/* Details Panel */}
          <div className="space-y-6">
            <AnimatePresence mode="wait">
              <div className="bg-white/5 border border-white/10 rounded-3xl p-6 border-dashed flex flex-col items-center justify-center h-64 text-center">
                <p className="text-gray-500 text-sm italic">The Avogadro Matrix is active. Neural resonance stable.</p>
              </div>
            </AnimatePresence>

            {/* Ingestion Stream Placeholder */}
            <div className="bg-white/5 border border-white/10 rounded-3xl p-6">
              <h3 className="text-sm font-bold uppercase tracking-widest text-gray-400 mb-4 flex items-center gap-2">
                <Activity className="w-4 h-4" />
                Ingestion Stream
              </h3>
              <div className="space-y-3">
                <StreamItem label="Wikipedia (Simple)" status="Active" progress={45} />
                <StreamItem label="ArXiv (Physics)" status="Queued" progress={0} />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function StatCard({ icon, label, value, color = 'text-white' }: { icon: React.ReactNode, label: string, value: string, color?: string }) {
  return (
    <div className="px-4 py-2 bg-white/5 rounded-2xl border border-white/10 flex items-center gap-3">
      <div className="text-gray-500">{icon}</div>
      <div>
        <div className="text-[10px] uppercase font-bold text-gray-500 leading-none mb-1">{label}</div>
        <div className={`text-sm font-mono font-bold ${color}`}>{value}</div>
      </div>
    </div>
  );
}



function StreamItem({ label, status, progress }: { label: string, status: string, progress: number }) {
  return (
    <div className="space-y-2">
      <div className="flex justify-between text-[10px] font-bold uppercase">
        <span className="text-gray-200">{label}</span>
        <span className={status === 'Active' ? 'text-blue-400' : 'text-gray-500'}>{status}</span>
      </div>
      <div className="h-1 bg-white/5 rounded-full overflow-hidden">
        <div className="h-full bg-blue-500" style={{ width: `${progress}%` }} />
      </div>
    </div>
  );
}
