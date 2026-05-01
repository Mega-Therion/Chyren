'use client';

import React, { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Activity, ShieldCheck, Zap } from 'lucide-react';

interface Metrics {
  chyren_task_admitted_total?: number;
  chyren_active_runs?: number;
  chyren_adccl_score?: number;
}

export const MetricsDashboard: React.FC = () => {
  const [data, setData] = useState<{ metrics: Metrics; timestamp: string } | null>(null);
  const [error, setError] = useState<string | null>(null);

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
    const interval = setInterval(fetchMetrics, 2000);
    return () => clearInterval(interval);
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
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 w-full p-4">
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
