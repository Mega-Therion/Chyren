'use client';

import React, { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Activity, ShieldCheck, Zap } from 'lucide-react';

interface Metrics {
  chyren_task_admitted_total?: number;
  chyren_active_runs?: number;
  chyren_adccl_score?: number;
}

interface SystemEvent {
  component: string;
  event_type: string;
  level: 'Info' | 'Warn' | 'Critical';
  payload: Record<string, unknown>;
  timestamp: number;
}

export const MetricsDashboard: React.FC = () => {
  const [data, setData] = useState<{ metrics: Metrics; timestamp: string } | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [logs, setLogs] = useState<SystemEvent[]>([]);

  const fetchMetrics = async () => {
    try {
      const res = await fetch('/api/metrics');
      if (res.ok) {
        const json = await res.json();
        setData(json);
        setError(null);
      } else {
        setError('Medulla Offline');
      }
    } catch {
      setError('Connection Error');
    }
  };

  useEffect(() => {
    void fetchMetrics();
    const interval = setInterval(fetchMetrics, 30000);
    
    // Connect to WebSocket for real-time events
    const wsUrl = process.env.NEXT_PUBLIC_MEDULLA_WS_URL || 'ws://localhost:9090/ws';
    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      try {
        const payload: SystemEvent = JSON.parse(event.data);
        
        // Add to logs stream
        setLogs(prev => [payload, ...prev].slice(0, 50));
        
        // Dynamically update metrics based on known event types
        if (payload.event_type === 'TaskAdmitted') {
          setData(prev => prev ? {
            ...prev,
            metrics: {
              ...prev.metrics,
              chyren_task_admitted_total: (prev.metrics.chyren_task_admitted_total || 0) + 1,
              chyren_active_runs: (prev.metrics.chyren_active_runs || 0) + 1
            }
          } : null);
        } else if (payload.event_type === 'AdcclEvaluated') {
            setData(prev => prev ? {
              ...prev,
              metrics: {
                ...prev.metrics,
                chyren_adccl_score: (payload.payload as Record<string, number>).score ?? prev.metrics.chyren_adccl_score
              }
            } : null);
        } else if (payload.event_type === 'TaskCompleted') {
           setData(prev => prev ? {
              ...prev,
              metrics: {
                ...prev.metrics,
                chyren_active_runs: Math.max(0, (prev.metrics.chyren_active_runs || 1) - 1)
              }
            } : null);
        }

      } catch (e) {
        console.error("Failed to parse WS telemetry:", e);
      }
    };

    return () => {
      clearInterval(interval);
      ws.close();
    };
  }, []);

  if (error) {
    return (
      <div className="p-4 bg-red-900/20 border border-red-500/50 rounded-xl flex items-center gap-3 text-red-200 text-sm">
        <Activity className="w-4 h-4 animate-pulse" />
        {error}
      </div>
    );
  }

  const metrics = data?.metrics || {};

  return (
    <div className="flex flex-col gap-4 w-full p-4">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <MetricCard 
          label="Task Admission" 
          value={metrics.chyren_task_admitted_total || 0} 
          icon={<Zap className="w-5 h-5 text-yellow-400" />}
          color="from-yellow-400/20 to-orange-500/20"
        />
        <MetricCard 
          label="Active Runs" 
          value={metrics.chyren_active_runs || 0} 
          icon={<Activity className="w-5 h-5 text-blue-400" />}
          color="from-blue-400/20 to-indigo-500/20"
        />
        <MetricCard 
          label="ADCCL Alignment" 
          value={`${((metrics.chyren_adccl_score || 0) * 100).toFixed(1)}%`} 
          icon={<ShieldCheck className="w-5 h-5 text-green-400" />}
          color="from-green-400/20 to-emerald-500/20"
        />
      </div>

      <div className="bg-zinc-900/40 backdrop-blur-xl border border-white/5 rounded-lg p-4 flex flex-col gap-2 shadow-2xl h-64 overflow-y-auto">
        <div className="flex items-center justify-between mb-2 border-b border-white/5 pb-2">
          <span className="text-[10px] font-mono uppercase tracking-widest text-zinc-500">Live Execution Log Stream</span>
          <span className="flex h-2 w-2 relative">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
          </span>
        </div>
        <AnimatePresence>
          {logs.map((log, i) => (
            <motion.div
              key={`${log.timestamp}-${i}`}
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
              className="text-xs font-mono py-1 border-b border-white/5 last:border-0 flex gap-2"
            >
              <span className="text-zinc-500">[{new Date(log.timestamp * 1000).toISOString().split('T')[1].replace('Z', '')}]</span>
              <span className={`font-semibold ${log.level === 'Critical' ? 'text-red-400' : log.level === 'Warn' ? 'text-yellow-400' : 'text-blue-400'}`}>
                [{log.level}]
              </span>
              <span className="text-purple-400">[{log.component}]</span>
              <span className="text-zinc-300">{log.event_type}</span>
              <span className="text-zinc-500 truncate ml-auto max-w-[50%]">{JSON.stringify(log.payload)}</span>
            </motion.div>
          ))}
          {logs.length === 0 && (
            <div className="text-xs font-mono text-zinc-600 italic">Waiting for events...</div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};

const MetricCard: React.FC<{ label: string, value: string | number, icon: React.ReactNode, color: string }> = ({ label, value, icon, color }) => (
  <motion.div 
    initial={{ opacity: 0, y: 10 }}
    animate={{ opacity: 1, y: 0 }}
    className={`bg-zinc-900/40 backdrop-blur-xl border border-white/5 rounded-lg p-4 flex flex-col gap-2 shadow-2xl transition-all hover:border-white/20 group`}
  >
    <div className="flex items-center justify-between">
      <span className="text-[10px] font-mono uppercase tracking-widest text-zinc-500 group-hover:text-zinc-300 transition-colors">{label}</span>
      <div className="opacity-50 group-hover:opacity-100 transition-opacity">
        {icon}
      </div>
    </div>
    <AnimatePresence mode="wait">
      <motion.span 
        key={value}
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 1.05 }}
        className="text-2xl font-bold font-mono text-zinc-100"
      >
        {value}
      </motion.span>
    </AnimatePresence>
    <div className={`h-0.5 w-0 group-hover:w-full transition-all duration-500 bg-gradient-to-r ${color}`} />
  </motion.div>
);
