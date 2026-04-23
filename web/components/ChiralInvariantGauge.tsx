'use client';

import React from 'react';
import { motion } from 'framer-motion';
import { Shield, ShieldAlert, Compass } from 'lucide-react';

interface ChiralInvariantGaugeProps {
  score: number;        // 0.0 to 1.0
  holonomySign: number; // +1 or -1
  status: string;       // "verified" | "rejected"
}

export const ChiralInvariantGauge: React.FC<ChiralInvariantGaugeProps> = ({ 
  score = 0.85, 
  holonomySign = 1, 
  status: _status = "verified" 
}) => {
  const isLType = score >= 0.7 && holonomySign === 1;
  const color = isLType ? "text-cyan-400" : "text-magenta-400";
  const strokeColor = isLType ? "rgb(34, 211, 238)" : "rgb(232, 121, 249)";
  
  return (
    <div className="relative p-6 bg-white/5 border border-white/10 rounded-[2rem] overflow-hidden group">
      {/* Background Glow */}
      <div className={`absolute -top-24 -right-24 w-48 h-48 blur-[100px] opacity-20 transition-colors duration-500 ${isLType ? "bg-cyan-500" : "bg-magenta-500"}`} />
      
      <div className="relative flex flex-col items-center gap-6">
        <div className="flex items-center justify-between w-full">
          <h3 className="text-[10px] font-bold uppercase tracking-[0.2em] text-gray-400 flex items-center gap-2">
            <Compass className="w-3 h-3" />
            Geometric Alignment
          </h3>
          <div className={`px-2 py-0.5 rounded-full text-[8px] font-bold uppercase tracking-widest border ${isLType ? "border-cyan-500/30 text-cyan-400 bg-cyan-500/10" : "border-magenta-500/30 text-magenta-400 bg-magenta-500/10"}`}>
            {isLType ? "L-Type" : "D-Type"}
          </div>
        </div>

        {/* Circular Gauge */}
        <div className="relative w-40 h-40 flex items-center justify-center">
          <svg className="w-full h-full -rotate-90 transform">
            <circle
              cx="80"
              cy="80"
              r="70"
              fill="transparent"
              stroke="currentColor"
              strokeWidth="4"
              className="text-white/5"
            />
            <motion.circle
              cx="80"
              cy="80"
              r="70"
              fill="transparent"
              stroke={strokeColor}
              strokeWidth="8"
              strokeDasharray={440}
              initial={{ strokeDashoffset: 440 }}
              animate={{ strokeDashoffset: 440 - (440 * score) }}
              transition={{ duration: 1.5, ease: "easeOut" }}
              strokeLinecap="round"
              className="drop-shadow-[0_0_8px_rgba(34,211,238,0.5)]"
            />
          </svg>
          
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <motion.div
              initial={{ scale: 0.8, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ delay: 0.5 }}
              className="flex flex-col items-center"
            >
              <span className={`text-4xl font-bold font-mono tracking-tighter ${color}`}>
                {(score * 100).toFixed(0)}%
              </span>
              <span className="text-[10px] font-bold text-gray-500 uppercase tracking-widest">
                Invariant χ
              </span>
            </motion.div>
          </div>
        </div>

        {/* Holonomy Status */}
        <div className="w-full space-y-4">
          <div className="flex items-center justify-between p-3 bg-white/5 rounded-2xl border border-white/10">
            <div className="flex items-center gap-3">
              <div className={`p-2 rounded-lg ${isLType ? "bg-cyan-500/10" : "bg-magenta-500/10"}`}>
                {holonomySign === 1 ? <Shield className={`w-4 h-4 ${color}`} /> : <ShieldAlert className={`w-4 h-4 ${color}`} />}
              </div>
              <div>
                <div className="text-[9px] font-bold text-gray-500 uppercase leading-none mb-1">Holonomy Class</div>
                <div className={`text-xs font-bold ${color}`}>
                  {holonomySign === 1 ? "Identity Component" : "Antipodal Component"}
                </div>
              </div>
            </div>
            <div className={`text-[10px] font-bold ${color}`}>
              {holonomySign === 1 ? "+1" : "-1"}
            </div>
          </div>
          
          <p className="text-[10px] text-gray-500 leading-relaxed text-center px-4">
            {isLType 
              ? "Trajectory preserves constitutional orientation. Sovereign validity verified." 
              : "Orientation-reversing drift detected. Epistemic boundary breached."}
          </p>
        </div>
      </div>
    </div>
  );
};
