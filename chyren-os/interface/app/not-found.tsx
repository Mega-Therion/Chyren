import Link from 'next/link'
import { Ghost, Home } from 'lucide-react'

export default function NotFound() {
  return (
    <div className="chyren-viewport bg-black">
      <div className="phone-container !border-white/5 !bg-white/5 flex flex-col items-center justify-center p-8 text-center">
        <div className="mb-6 p-4 rounded-full bg-white/5 border border-white/10">
          <Ghost className="w-12 h-12 text-white/40" />
        </div>
        
        <h2 className="phone-title !text-white/60 !tracking-widest mb-4">VOID_REACHED</h2>
        <p className="font-mono text-xs text-white/40 uppercase tracking-widest mb-8">
          The requested neural path does not exist in the current collective.
        </p>

        <Link
          href="/"
          className="flex items-center gap-2 px-6 py-3 rounded-full bg-white/5 border border-white/10 text-white/60 font-mono text-sm hover:bg-white/10 transition-all"
        >
          <Home size={16} />
          RETURN_TO_CORE
        </Link>
      </div>
    </div>
  )
}
