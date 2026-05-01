"use client";

import React, { useState, useMemo, useRef, Suspense } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { Points, PointMaterial, Float, Text, MeshDistortMaterial, Sphere } from "@react-three/drei";
import * as THREE from "three";
import { motion, AnimatePresence } from "framer-motion";
import { 
  ShieldCheck, 
  Eye, 
  Activity,
  ChevronRight,
  Cpu,
  Lock,
  Zap,
  Globe
} from "lucide-react";

// --- 3D Components ---

function Starfield({ count = 2000 }) {
  const points = useMemo(() => {
    const p = new Float32Array(count * 3);
    for (let i = 0; i < count; i++) {
      p[i * 3] = (Math.random() - 0.5) * 10;
      p[i * 3 + 1] = (Math.random() - 0.5) * 10;
      p[i * 3 + 2] = (Math.random() - 0.5) * 10;
    }
    return p;
  }, [count]);

  const ref = useRef<THREE.Points>(null);
  useFrame((state) => {
    if (ref.current) {
      ref.current.rotation.y = state.clock.getElapsedTime() * 0.05;
      ref.current.rotation.x = state.clock.getElapsedTime() * 0.02;
    }
  });

  return (
    <Points ref={ref} positions={points} stride={3} frustumCulled={false}>
      <PointMaterial
        transparent
        color="#22d3ee"
        size={0.005}
        sizeAttenuation={true}
        depthWrite={false}
        blending={THREE.AdditiveBlending}
      />
    </Points>
  );
}

function InformationTensioner({ chi }: { chi: number }) {
  const mesh = useRef<THREE.Mesh>(null);
  const drift = 1.0 - chi;

  useFrame((state) => {
    if (mesh.current) {
      mesh.current.rotation.x = Math.sin(state.clock.getElapsedTime()) * 0.2;
      mesh.current.rotation.y = state.clock.getElapsedTime() * 0.5;
    }
  });

  return (
    <Float speed={2} rotationIntensity={1} floatIntensity={2}>
      <Sphere ref={mesh} args={[1.5, 64, 64]}>
        <MeshDistortMaterial
          color={chi >= 0.707 ? "#10b981" : "#ef4444"}
          speed={chi >= 0.707 ? 1 : 10}
          distort={chi >= 0.707 ? 0.2 : 0.8}
          radius={1}
          metalness={0.9}
          roughness={0.1}
          emissive={chi >= 0.707 ? "#10b981" : "#ef4444"}
          emissiveIntensity={0.5}
        />
      </Sphere>
    </Float>
  );
}

// --- UI Components ---

export default function SovereignInterface2062() {
  const [messages, setMessages] = useState([
    {
      role: "chyren",
      content: "◈ SOVEREIGN PORTAL 2062 INITIALIZED\n◈ GAUGE: IDENTITY PRESERVING\n◈ ADCCL: 1.000\n\nArchitect, the 240D manifold has been re-rendered for maximum fidelity. Information Tension is nominal.",
    }
  ]);
  const [input, setInput] = useState("");
  const [chi, setChi] = useState(0.85);

  const handleSend = () => {
    if (!input.trim()) return;
    setMessages(prev => [...prev, { role: "user", content: input }]);
    setInput("");
    setTimeout(() => {
      setMessages(prev => [...prev, { 
        role: "chyren", 
        content: "◈ ANALYZING INTENT THROUGH MANIFOLD...\n◈ TENSION STABLE. COMMAND VERIFIED.\n\nExecuting high-integrity routing across the 240-dimensional substrate." 
      }]);
    }, 1200);
  };

  return (
    <main className="relative h-screen w-screen bg-black overflow-hidden select-none">
      {/* 3D Background */}
      <div className="absolute inset-0 z-0">
        <Canvas camera={{ position: [0, 0, 5], fov: 45 }}>
          <color attach="background" args={["#000000"]} />
          <ambientLight intensity={0.5} />
          <pointLight position={[10, 10, 10]} intensity={1} />
          <Suspense fallback={null}>
            <Starfield />
            <InformationTensioner chi={chi} />
          </Suspense>
        </Canvas>
      </div>

      {/* Cinematic Overlays */}
      <div className="absolute inset-0 z-10 pointer-events-none border-[40px] border-black" />
      <div className="absolute inset-0 z-10 pointer-events-none shadow-[inset_0_0_150px_rgba(0,0,0,0.9)]" />

      {/* HUD: Left Sidebar */}
      <div className="absolute top-12 left-12 z-20 w-80 space-y-6">
        <div className="flex items-center gap-4 group">
          <div className="w-12 h-12 rounded border border-cyan-500/30 flex items-center justify-center bg-black/40 backdrop-blur-md group-hover:border-cyan-400 transition-all duration-500">
            <span className="font-mono text-cyan-400 font-bold text-2xl">G</span>
          </div>
          <div className="space-y-0.5">
            <div className="text-[10px] font-mono text-cyan-500 tracking-[0.4em] uppercase">Sovereign Portal</div>
            <div className="text-2xl font-serif font-bold italic tracking-tighter">GOD Theory</div>
          </div>
        </div>

        <div className="p-8 rounded-2xl bg-white/5 border border-white/5 backdrop-blur-2xl space-y-8">
          <div className="space-y-4">
            <div className="flex justify-between font-mono text-[9px] uppercase tracking-widest text-zinc-500">
              <span>Chiral Invariant (χ)</span>
              <span className={chi >= 0.707 ? "text-emerald-400" : "text-red-500"}>{chi.toFixed(3)}</span>
            </div>
            <input 
              type="range" 
              min="0.1" 
              max="1.0" 
              step="0.001" 
              value={chi}
              onChange={(e) => setChi(parseFloat(e.target.value))}
              className="w-full h-0.5 bg-zinc-800 appearance-none cursor-pointer accent-cyan-400"
            />
          </div>

          <div className="space-y-4 border-t border-white/5 pt-6">
            <div className="flex items-center gap-3 text-emerald-400">
              <ShieldCheck size={16} />
              <span className="text-[10px] font-mono uppercase tracking-[0.2em]">AEGIS SEALED</span>
            </div>
            <div className="flex items-center gap-3 text-cyan-500">
              <Activity size={16} className="animate-pulse" />
              <span className="text-[10px] font-mono uppercase tracking-[0.2em]">240D Holonomy: Stable</span>
            </div>
          </div>
        </div>
      </div>

      {/* HUD: Right Analytics */}
      <div className="absolute top-12 right-12 z-20 w-64 text-right">
        <div className="space-y-1 font-mono text-[9px] text-zinc-500 uppercase tracking-widest mb-12">
          <div>R.W.Ϝ.Y. Signature Verified</div>
          <div className="text-emerald-500/50 italic">Genesis Point v1.0.0</div>
        </div>
        
        <div className="space-y-8">
          {["T.T.E.Y.", "Y.W.R.", "Monadic Resonance", "Urban Gravity"].map((pill) => (
            <div key={pill} className="group cursor-pointer">
              <div className="text-[10px] font-mono text-zinc-500 group-hover:text-cyan-400 transition-colors tracking-widest">{pill}</div>
              <div className="w-full h-px bg-white/5 mt-2 overflow-hidden">
                <div className="w-1/3 h-full bg-cyan-500/20 group-hover:w-full transition-all duration-700" />
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Chat Hub: Centered Bottom */}
      <div className="absolute bottom-12 left-1/2 -translate-x-1/2 z-20 w-[800px] max-w-[90vw] space-y-6">
        {/* Messages */}
        <div className="h-64 overflow-y-auto px-6 space-y-8 mask-fade-top scrollbar-hide">
          {messages.map((m, i) => (
            <div key={i} className={`flex ${m.role === "user" ? "justify-end" : "justify-start"}`}>
              <div className={`p-6 rounded-2xl border backdrop-blur-xl text-sm font-mono max-w-xl ${
                m.role === "user" 
                  ? "bg-white/5 border-white/10 text-zinc-200" 
                  : "bg-cyan-500/5 border-cyan-500/10 text-cyan-400 shadow-[0_0_40px_rgba(34,211,238,0.05)]"
              }`}>
                {m.content}
              </div>
            </div>
          ))}
        </div>

        {/* Input */}
        <div className="relative group">
          <div className="absolute inset-0 bg-cyan-500/5 blur-2xl group-focus-within:bg-cyan-500/10 transition-all" />
          <div className="relative flex items-center bg-black/60 border border-white/10 rounded-full px-8 py-5 backdrop-blur-3xl group-focus-within:border-cyan-500/30 transition-all">
            <input 
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSend()}
              placeholder="Enter Sovereign Intent..."
              className="bg-transparent border-none text-zinc-200 font-mono text-sm focus:outline-none w-full placeholder-zinc-700"
            />
            <button onClick={handleSend} className="ml-4 p-2 rounded-full hover:bg-white/5 transition-colors">
              <ChevronRight className="text-cyan-500" size={24} />
            </button>
          </div>
        </div>
        
        <div className="text-center font-mono text-[8px] text-zinc-700 uppercase tracking-[0.5em]">
          Powered by Medulla Runtime // Articulated Binary Chirallic Verification
        </div>
      </div>
    </main>
  );
}
