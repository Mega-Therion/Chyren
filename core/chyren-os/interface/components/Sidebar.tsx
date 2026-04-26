'use client'

import React, { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'

interface SidebarProps {
  isOpen: boolean
  onToggle: () => void
  brainState: string
  sessionId: string
  onNewChat: () => void
}

const NAV_ITEMS = [
  { id: 'chat', icon: '⟐', label: 'Neural Link', shortcut: '⌘1' },
  { id: 'ledger', icon: '⛓', label: 'Master Ledger', shortcut: '⌘2' },
  { id: 'metrics', icon: '◈', label: 'Telemetry', shortcut: '⌘3' },
  { id: 'memory', icon: '⧠', label: 'Myelin Store', shortcut: '⌘4' },
]

const QUICK_ACTIONS = [
  { id: 'recon', icon: '◉', label: 'Recon', color: '#bc13fe' },
  { id: 'dream', icon: '☽', label: 'Dream', color: '#6366f1' },
  { id: 'voice', icon: '⏦', label: 'Voice', color: '#00f2ff' },
  { id: 'deploy', icon: '▲', label: 'Deploy', color: '#39ff14' },
]

export function Sidebar({ isOpen, onToggle, brainState, sessionId, onNewChat }: SidebarProps) {
  const [activeNav, setActiveNav] = useState('chat')

  const stateColors: Record<string, string> = {
    idle: 'rgba(255, 255, 255, 0.2)',
    listening: 'var(--chyren-violet)',
    thinking: 'var(--chyren-gold)',
    speaking: 'var(--chyren-blue)',
  }

  const stateLabels: Record<string, string> = {
    idle: 'IDLE',
    listening: 'LISTENING',
    thinking: 'REASONING',
    speaking: 'TRANSMITTING',
  }

  return (
    <>
      <AnimatePresence>
        {isOpen && (
          <motion.aside
            className="fixed left-0 top-0 bottom-0 w-[280px] bg-black border-r border-white/5 z-50 flex flex-col"
            initial={{ x: -280 }}
            animate={{ x: 0 }}
            exit={{ x: -280 }}
            transition={{ type: 'spring', damping: 30, stiffness: 300 }}
          >
            {/* Minimal Identity */}
            <div className="p-10 pb-6 flex flex-col gap-6">
              <div className="flex items-center gap-4">
                <div className="w-10 h-10 rounded-full bg-white/5 flex items-center justify-center text-lg font-thin border border-white/10">
                  Ω
                </div>
                <div className="flex flex-col">
                  <span className="text-[10px] tracking-[0.4em] font-bold text-white/80 uppercase">Chyren</span>
                  <div className="flex items-center gap-2 mt-1">
                    <motion.div 
                      className="w-1 h-1 rounded-full"
                      animate={{ backgroundColor: stateColors[brainState], scale: [1, 1.5, 1] }}
                      transition={{ repeat: Infinity, duration: 2 }}
                    />
                    <span className="text-[8px] tracking-[0.2em] font-mono text-white/30 uppercase">{stateLabels[brainState]}</span>
                  </div>
                </div>
              </div>
              
              <button 
                className="w-full py-3 px-4 rounded-xl bg-white/5 hover:bg-white/10 border border-white/5 transition-all text-left flex items-center justify-between group"
                onClick={onNewChat}
              >
                <span className="text-[10px] tracking-[0.2em] font-medium text-white/60 group-hover:text-white uppercase">New Session</span>
                <span className="text-white/20 group-hover:text-white/60">+</span>
              </button>
            </div>

            {/* Navigation */}
            <nav className="flex-1 px-6 space-y-2">
              {NAV_ITEMS.map(item => (
                <button
                  key={item.id}
                  className={`w-full flex items-center gap-4 px-4 py-3 rounded-xl transition-all group ${activeNav === item.id ? 'bg-white/5 text-white' : 'text-white/40 hover:text-white/60'}`}
                  onClick={() => setActiveNav(item.id)}
                >
                  <span className="text-sm opacity-60 group-hover:opacity-100">{item.icon}</span>
                  <span className="text-[11px] tracking-[0.1em] font-medium uppercase">{item.label}</span>
                </button>
              ))}
            </nav>

            {/* Quick Actions Tray */}
            <div className="p-6 grid grid-cols-4 gap-2">
              {QUICK_ACTIONS.map(action => (
                <button
                  key={action.id}
                  className="aspect-square rounded-xl bg-white/5 hover:bg-white/10 flex items-center justify-center transition-all group"
                  title={action.label}
                >
                  <span className="text-sm opacity-40 group-hover:opacity-100" style={{ color: action.color }}>{action.icon}</span>
                </button>
              ))}
            </div>

            {/* Subtle Footer */}
            <div className="p-8 space-y-4">
              <div className="flex flex-col gap-2">
                <div className="flex justify-between items-center text-[8px] tracking-[0.3em] font-mono">
                  <span className="text-white/20 uppercase">Session ID</span>
                  <span className="text-white/40 uppercase">{sessionId.slice(0, 8)}</span>
                </div>
                <div className="flex justify-between items-center text-[8px] tracking-[0.3em] font-mono">
                  <span className="text-white/20 uppercase">ADCCL Score</span>
                  <span className="text-cyan-400/60 uppercase">0.9928</span>
                </div>
              </div>
            </div>
          </motion.aside>
        )}
      </AnimatePresence>

      {/* Collapse Trigger (Floating) */}
      {!isOpen && (
        <button
          onClick={onToggle}
          className="fixed left-6 top-1/2 -translate-y-1/2 w-8 h-12 bg-white/5 hover:bg-white/10 border border-white/5 rounded-full flex items-center justify-center transition-all z-50 group"
        >
          <span className="text-white/20 group-hover:text-white/60">›</span>
        </button>
      )}
      
      {isOpen && (
        <button
          onClick={onToggle}
          className="fixed left-[280px] top-1/2 -translate-y-1/2 w-8 h-12 bg-black border border-white/5 border-l-0 rounded-r-full flex items-center justify-center transition-all z-50 group"
        >
          <span className="text-white/20 group-hover:text-white/60">‹</span>
        </button>
      )}
    </>
  )
}
