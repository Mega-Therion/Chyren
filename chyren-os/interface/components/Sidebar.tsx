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
    idle: '#f59e0b',
    listening: '#bc13fe',
    thinking: '#ff2d75',
    speaking: '#00f2ff',
  }

  const stateLabels: Record<string, string> = {
    idle: 'STANDBY',
    listening: 'LISTENING',
    thinking: 'REASONING',
    speaking: 'TRANSMITTING',
  }

  return (
    <>
      {/* Collapsed toggle button */}
      <button
        onClick={onToggle}
        className="sidebar-toggle"
        aria-label="Toggle sidebar"
        style={{ left: isOpen ? '280px' : '0' }}
      >
        <motion.span
          animate={{ rotate: isOpen ? 180 : 0 }}
          transition={{ duration: 0.2 }}
          style={{ display: 'inline-block', fontSize: '0.75rem', color: '#f59e0b' }}
        >
          {isOpen ? '◂' : '▸'}
        </motion.span>
      </button>

      <AnimatePresence>
        {isOpen && (
          <motion.aside
            className="sidebar"
            initial={{ x: -280, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            exit={{ x: -280, opacity: 0 }}
            transition={{ type: 'spring', damping: 25, stiffness: 200 }}
          >
            {/* Identity Badge */}
            <div className="sidebar-identity">
              <div className="sidebar-avatar">
                <motion.div
                  className="sidebar-avatar-ring"
                  animate={{
                    boxShadow: `0 0 ${brainState === 'idle' ? '12' : '24'}px ${stateColors[brainState] || '#f59e0b'}40`,
                    borderColor: stateColors[brainState] || '#f59e0b',
                  }}
                  transition={{ duration: 0.6 }}
                >
                  <span className="sidebar-avatar-sigil">Ω</span>
                </motion.div>
              </div>
              <div className="sidebar-identity-text">
                <h2 className="sidebar-identity-name">CHYREN</h2>
                <div className="sidebar-identity-status">
                  <motion.span
                    className="sidebar-status-dot"
                    animate={{
                      backgroundColor: stateColors[brainState] || '#f59e0b',
                      scale: brainState === 'idle' ? [1, 1.2, 1] : [1, 1.4, 1],
                    }}
                    transition={{ repeat: Infinity, duration: brainState === 'idle' ? 3 : 1.2 }}
                  />
                  <span style={{ color: stateColors[brainState] || '#f59e0b' }}>
                    {stateLabels[brainState] || 'STANDBY'}
                  </span>
                </div>
              </div>
            </div>

            {/* New Chat Button */}
            <button className="sidebar-new-chat" onClick={onNewChat}>
              <span style={{ fontSize: '0.85rem' }}>⊕</span>
              <span>New Session</span>
            </button>

            {/* Navigation */}
            <nav className="sidebar-nav">
              {NAV_ITEMS.map(item => (
                <button
                  key={item.id}
                  className={`sidebar-nav-item ${activeNav === item.id ? 'sidebar-nav-item--active' : ''}`}
                  onClick={() => setActiveNav(item.id)}
                >
                  <span className="sidebar-nav-icon">{item.icon}</span>
                  <span className="sidebar-nav-label">{item.label}</span>
                  <span className="sidebar-nav-shortcut">{item.shortcut}</span>
                </button>
              ))}
            </nav>

            {/* Quick Actions */}
            <div className="sidebar-section-title">QUICK ACTIONS</div>
            <div className="sidebar-actions">
              {QUICK_ACTIONS.map(action => (
                <button
                  key={action.id}
                  className="sidebar-action-btn"
                  title={action.label}
                >
                  <span style={{ color: action.color, fontSize: '1rem' }}>{action.icon}</span>
                  <span className="sidebar-action-label">{action.label}</span>
                </button>
              ))}
            </div>

            {/* Session Info */}
            <div className="sidebar-footer">
              <div className="sidebar-session-info">
                <span className="sidebar-session-label">SESSION</span>
                <span className="sidebar-session-id">{sessionId.slice(0, 8)}</span>
              </div>
              <div className="sidebar-session-info">
                <span className="sidebar-session-label">ADCCL</span>
                <span className="sidebar-session-id" style={{ color: '#39ff14' }}>0.950</span>
              </div>
              <div className="sidebar-session-info">
                <span className="sidebar-session-label">INTEGRITY</span>
                <span className="sidebar-session-id" style={{ color: '#39ff14' }}>VERIFIED</span>
              </div>
            </div>
          </motion.aside>
        )}
      </AnimatePresence>
    </>
  )
}
