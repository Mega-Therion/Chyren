'use client'

import React from 'react'
import { motion, AnimatePresence } from 'framer-motion'

interface MissionControlProps {
  brainState: 'idle' | 'thinking' | 'speaking' | 'listening'
  sessionId: string
}

export function MissionControl({ brainState, sessionId }: MissionControlProps) {
  const getColor = () => {
    switch (brainState) {
      case 'thinking': return 'var(--chyren-gold)'
      case 'speaking': return 'var(--chyren-blue)'
      case 'listening': return 'var(--chyren-violet)'
      default: return 'rgba(255,255,255,0.2)'
    }
  }

  return (
    <div className="fixed top-8 right-10 z-50 flex flex-col items-end gap-3 pointer-events-none">
      <div className="flex items-center gap-4">
        <span className="text-[8px] font-mono text-white/10 tracking-[0.4em] uppercase">
          PROVENANCE::{sessionId.slice(0, 8)}
        </span>
        <div className="relative w-2 h-2">
          <AnimatePresence mode="wait">
            <motion.div
              key={brainState}
              className="absolute inset-0 rounded-full"
              style={{ backgroundColor: getColor() }}
              initial={{ scale: 0.5, opacity: 0 }}
              animate={{ 
                scale: [1, 1.5, 1],
                opacity: 1,
                boxShadow: `0 0 15px ${getColor()}80`
              }}
              transition={{ 
                repeat: Infinity, 
                duration: brainState === 'idle' ? 3 : 1.5,
                ease: "easeInOut"
              }}
            />
          </AnimatePresence>
        </div>
      </div>
      
      {/* Ultra-minimal status line */}
      <motion.div 
        className="h-[1px] bg-gradient-to-l from-white/20 to-transparent"
        animate={{ width: brainState === 'idle' ? 40 : 120 }}
        style={{ backgroundColor: getColor() + '40' }}
      />
    </div>
  )
}
