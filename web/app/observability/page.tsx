'use client';

import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { motion, AnimatePresence } from 'framer-motion';
import { Eye, Database, Activity, ShieldCheck, Zap } from 'lucide-react';

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

const REALM_COLORS: Record<string, string> = {
  sovereign: '#F59E0B', // Gold
  people: '#10B981',    // Emerald
  external: '#3B82F6',  // Blue
  unknown: '#6B7280',   // Gray
};

export default function ObservabilityPage() {
  const [stats, setStats] = useState<Stats | null>(null);
  const [selectedPoint, setSelectedPoint] = useState<Point | null>(null);
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    fetch('/api/observability/stats')
      .then((res) => res.json())
      .then((data) => setStats(data))
      .catch((err) => console.error(err));
  }, []);

  useEffect(() => {
    if (!stats || !svgRef.current) return;

    const width = 800;
    const height = 500;
    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove();

    const nodes = stats.points.map((p) => ({
      id: p.id,
      name: p.payload.name,
      realm: p.payload.realm,
      point: p,
    }));

    const simulation = d3.forceSimulation(nodes as any)
      .force('charge', d3.forceManyBody().strength(-30))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(10))
      .on('tick', () => {
        node.attr('cx', (d: any) => d.x).attr('cy', (d: any) => d.y);
      });

    const node = svg
      .append('g')
      .selectAll('circle')
      .data(nodes)
      .enter()
      .append('circle')
      .attr('r', 6)
      .attr('fill', (d) => REALM_COLORS[d.realm] || REALM_COLORS.unknown)
      .attr('stroke', '#fff')
      .attr('stroke-width', 1.5)
      .style('cursor', 'pointer')
      .on('click', (event, d) => {
        setSelectedPoint(d.point);
      });

    node.append('title').text((d) => d.name);

  }, [stats]);

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
          <div className="lg:col-span-2 relative bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-xl">
            <div className="absolute top-4 left-4 z-10 flex gap-2">
              <LegendItem color="bg-amber-500" label="Sovereign" />
              <LegendItem color="bg-emerald-500" label="People" />
              <LegendItem color="bg-blue-500" label="External" />
            </div>
            <svg ref={svgRef} viewBox="0 0 800 500" className="w-full h-auto" />
            {!stats && (
              <div className="absolute inset-0 flex items-center justify-center bg-black/40 backdrop-blur-sm">
                <div className="flex flex-col items-center gap-4">
                  <Zap className="w-12 h-12 text-blue-400 animate-pulse" />
                  <p className="text-blue-400 font-medium">Synchronizing Matrix...</p>
                </div>
              </div>
            )}
          </div>

          {/* Details Panel */}
          <div className="space-y-6">
            <AnimatePresence mode="wait">
              {selectedPoint ? (
                <motion.div
                  key={selectedPoint.id}
                  initial={{ opacity: 0, x: 20 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: -20 }}
                  className="bg-white/5 border border-white/10 rounded-3xl p-6 backdrop-blur-xl h-full"
                >
                  <div className="flex items-center gap-2 mb-4">
                    <ShieldCheck className="w-5 h-5 text-emerald-400" />
                    <span className="text-xs font-bold uppercase tracking-widest text-emerald-400">ARI Verified</span>
                  </div>
                  <h3 className="text-2xl font-bold mb-2">{selectedPoint.payload.name}</h3>
                  <div className={`inline-block px-3 py-1 rounded-full text-xs font-bold uppercase mb-6 bg-opacity-20 ${
                    selectedPoint.payload.realm === 'sovereign' ? 'bg-amber-500 text-amber-400' :
                    selectedPoint.payload.realm === 'people' ? 'bg-emerald-500 text-emerald-400' :
                    'bg-blue-500 text-blue-400'
                  }`}>
                    {selectedPoint.payload.realm}
                  </div>
                  <div className="space-y-4">
                    <div>
                      <h4 className="text-sm font-medium text-gray-400 mb-1">Description</h4>
                      <p className="text-gray-200 leading-relaxed text-sm">
                        {selectedPoint.payload.description}
                      </p>
                    </div>
                  </div>
                </motion.div>
              ) : (
                <div className="bg-white/5 border border-white/10 rounded-3xl p-6 border-dashed flex flex-col items-center justify-center h-64 text-center">
                  <p className="text-gray-500 text-sm italic">Click a point in the matrix to view sovereign metadata.</p>
                </div>
              )}
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

function LegendItem({ color, label }: { color: string, label: string }) {
  return (
    <div className="flex items-center gap-2 px-2 py-1 bg-black/40 rounded-full border border-white/5 backdrop-blur-md">
      <div className={`w-2 h-2 rounded-full ${color}`} />
      <span className="text-[10px] font-bold uppercase text-gray-300">{label}</span>
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
