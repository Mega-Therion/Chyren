'use client'

import { useEffect } from 'react'
import { motion } from 'framer-motion'
import { AlertTriangle, RefreshCcw } from 'lucide-react'

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset: () => void
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error('Sovereign Hub Fault:', error)
  }, [error])

  return (
    <div className="omega-viewport bg-black">
      <div className="phone-container !border-rose-500/20 !bg-rose-950/10 flex flex-col items-center justify-center p-8 text-center">
        <motion.div
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          className="mb-6 p-4 rounded-full bg-rose-500/10 border border-rose-500/20"
        >
          <AlertTriangle className="w-12 h-12 text-rose-500" />
        </motion.div>
        
        <h2 className="phone-title !text-rose-500 !tracking-widest mb-4">CRITICAL_FAULT</h2>
        <p className="font-mono text-xs text-rose-400/60 uppercase tracking-widest mb-8">
          Neural link stability compromised: {error.message || 'Unknown corruption'}
        </p>

        <button
          onClick={() => reset()}
          className="flex items-center gap-2 px-6 py-3 rounded-full bg-rose-500/20 border border-rose-500/40 text-rose-400 font-mono text-sm hover:bg-rose-500/30 transition-all"
        >
          <RefreshCcw size={16} />
          REBOOT_LINK
        </button>
        
        <div className="mt-12 font-mono text-[10px] text-white/10 tracking-[0.3em] uppercase">
          Chyren // ADCCL Verification Active
        </div>
      </div>
    </div>
  )
}
