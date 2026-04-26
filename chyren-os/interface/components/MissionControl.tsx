'use client'

import React, { useEffect, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Brain, Cpu, Database, Activity, ShieldCheck, Zap } from 'lucide-react'

interface MissionControlProps {
  brainState: 'idle' | 'thinking' | 'speaking' | 'listening'
  sessionId: string
}

export function MissionControl({ brainState, sessionId }: MissionControlProps) {
  const [metrics, setMetrics] = useState({
    cpu: 0,
    mem: 0,
    latency: 0,
    integrity: 100
  })

  useEffect(() => {
    const timer = setInterval(() => {
      setMetrics({
        cpu: Math.floor(Math.random() * 15) + (brainState === 'thinking' ? 40 : 5),
        mem: Math.floor(Math.random() * 5) + 32,
        latency: Math.floor(Math.random() * 20) + (brainState === 'speaking' ? 120 : 45),
        integrity: 100 - (Math.random() * 0.1)
      })
    }, 2000)
    return () => clearInterval(timer)
  }, [brainState])

  const getColor = () => {
    switch (brainState) {
      case 'thinking': return '#ff2d75'
      case 'speaking': return '#00f2ff'
      case 'listening': return '#bc13fe'
      default: return '#f59e0b'
    }
  }

  return (
    <div className="fixed top-1/2 left-1/2 -translate-y-1/2 pointer-events-none z-10 flex items-center justify-center w-[600px] h-[600px]" style={{ left: 'calc(50% + 140px)' }}>
      {/* Neural Pulse Perimeter */}
      <div className="absolute inset-0 flex items-center justify-center">
        <motion.div
          className="absolute w-[480px] h-[480px] border border-dashed rounded-full"
          style={{ borderColor: `${getColor()}33`, boxShadow: `0 0 40px ${getColor()}22` }}
          animate={{ rotate: 360 }}
          transition={{ duration: 20, repeat: Infinity, ease: 'linear' }}
        />
        <motion.div
          className="absolute w-[440px] h-[440px] border-2 rounded-full border-y-transparent"
          style={{ borderColor: `${getColor()}1A` }}
          animate={{ rotate: -360, opacity: [0.2, 0.4, 0.2] }}
          transition={{ duration: 15, repeat: Infinity, ease: 'linear' }}
        />
      </div>

      {/* Stats Bento Grid */}
      <div className="absolute bottom-10 -right-16 pointer-events-auto grid grid-cols-2 gap-3 w-72">
        <StatCard
          icon={<Cpu size={14} />}
          label="NEURAL LOAD"
          value={`${metrics.cpu}%`}
          color={getColor()}
          progress={metrics.cpu}
        />
        <StatCard
          icon={<Activity size={14} />}
          label="LATENCY"
          value={`${metrics.latency}ms`}
          color={getColor()}
          progress={(metrics.latency / 300) * 100}
        />
        <StatCard
          icon={<Database size={14} />}
          label="MEMORY"
          value={`${metrics.mem}GB`}
          color={getColor()}
          progress={metrics.mem}
        />
        <StatCard
          icon={<ShieldCheck size={14} />}
          label="INTEGRITY"
          value={`${metrics.integrity.toFixed(2)}%`}
          color={getColor()}
          progress={metrics.integrity}
        />
      </div>

      {/* Session ID Tag */}
      <div className="absolute top-10 -left-16 flex items-center gap-3">
        <div className="w-10 h-px bg-gradient-to-r from-transparent to-white/20" />
        <span className="text-[10px] font-mono text-white/40 tracking-[0.2em]">SESSION::{sessionId.slice(0, 8).toUpperCase()}</span>
      </div>

      {/* Brain Icon Overlay */}
      <AnimatePresence mode="wait">
        <motion.div
          key={brainState}
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          exit={{ scale: 1.2, opacity: 0 }}
          className="relative z-20"
          style={{ color: getColor(), filter: `drop-shadow(0 0 20px ${getColor()})` }}
        >
          {brainState === 'thinking' ?
            <Zap className="animate-pulse" /> :
            <Brain className="animate-pulse" style={{ animationDuration: '3s' }} />
          }
        </motion.div>
      </AnimatePresence>
    </div>
  )
}

function StatCard({ icon, label, value, color, progress }: any) {
  return (
    <div className="bg-black/60 backdrop-blur-xl border border-white/5 p-3 rounded-xl flex flex-col gap-1.5 overflow-hidden">
      <div className="flex items-center gap-1.5 text-[9px] font-bold tracking-widest opacity-80" style={{ color }}>
        {icon}
        {label}
      </div>
      <div className="font-mono text-base font-semibold text-white">{value}</div>
      <div className="h-0.5 bg-white/5 rounded-full overflow-hidden mt-1">
        <motion.div
          className="h-full"
          style={{ backgroundColor: color }}
          animate={{ width: `${Math.min(progress, 100)}%` }}
        />
      </div>
    </div>
  )
}
