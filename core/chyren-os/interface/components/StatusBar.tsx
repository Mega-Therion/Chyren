'use client'

import React from 'react'
import { motion } from 'framer-motion'
import type { BrainState } from './NeuralBrain'

interface StatusBarProps {
  brainState: BrainState
  sessionId: string
  messageCount: number
  isStreaming: boolean
}

export function StatusBar({ brainState, sessionId, messageCount, isStreaming }: StatusBarProps) {
  const stateColors: Record<BrainState, string> = {
    idle: '#f59e0b',
    listening: '#bc13fe',
    thinking: '#ff2d75',
    speaking: '#00f2ff',
  }

  const stateLabels: Record<BrainState, string> = {
    idle: 'SOVEREIGN STANDBY',
    listening: 'VOICE CAPTURE',
    thinking: 'NEURAL REASONING',
    speaking: 'STREAM ACTIVE',
  }

  return (
    <footer className="status-bar">
      <div className="status-bar-inner">
        {/* Left: System state */}
        <div className="status-bar-section">
          <motion.div
            className="status-indicator"
            animate={{
              backgroundColor: stateColors[brainState],
              boxShadow: `0 0 8px ${stateColors[brainState]}80`,
              scale: isStreaming ? [1, 1.3, 1] : 1,
            }}
            transition={{ repeat: isStreaming ? Infinity : 0, duration: 0.8 }}
          />
          <span className="status-label" style={{ color: stateColors[brainState] }}>
            {stateLabels[brainState]}
          </span>
        </div>

        {/* Center: Pipeline stages */}
        <div className="status-bar-section status-bar-center">
          <span className="status-pipeline-stage" data-active={brainState === 'listening'}>
            STT
          </span>
          <span className="status-pipeline-sep">→</span>
          <span className="status-pipeline-stage" data-active={brainState === 'thinking'}>
            CORTEX
          </span>
          <span className="status-pipeline-sep">→</span>
          <span className="status-pipeline-stage" data-active={brainState === 'thinking'}>
            ADCCL
          </span>
          <span className="status-pipeline-sep">→</span>
          <span className="status-pipeline-stage" data-active={brainState === 'speaking'}>
            TTS
          </span>
        </div>

        {/* Right: Telemetry */}
        <div className="status-bar-section status-bar-right">
          <span className="status-metric">
            <span className="status-metric-label">MSGS</span>
            <span className="status-metric-value">{messageCount}</span>
          </span>
          <span className="status-metric">
            <span className="status-metric-label">SID</span>
            <span className="status-metric-value">{sessionId.slice(0, 6)}</span>
          </span>
          <span className="status-metric">
            <span className="status-metric-label">MODEL</span>
            <span className="status-metric-value" style={{ color: '#39ff14' }}>GROQ</span>
          </span>
        </div>
      </div>
    </footer>
  )
}
