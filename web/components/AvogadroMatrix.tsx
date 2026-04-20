'use client'

import React, { useRef, useMemo } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { OrbitControls, Sphere, MeshDistortMaterial, Float, Line } from '@react-three/drei'
import * as THREE from 'three'

// Mock Data for the Knowledge Matrix
const generateNodes = (count: number) => {
  return Array.from({ length: count }).map((_, i) => ({
    id: i,
    position: [
      (Math.random() - 0.5) * 15,
      (Math.random() - 0.5) * 15,
      (Math.random() - 0.5) * 15,
    ] as [number, number, number],
    size: Math.random() * 0.3 + 0.1,
    color: new THREE.Color().setHSL(Math.random(), 0.7, 0.5),
  }))
}

const Atom = ({ position, size, color }: { position: [number, number, number]; size: number; color: THREE.Color }) => {
  const meshRef = useRef<THREE.Mesh>(null!)
  
  useFrame((state) => {
    const t = state.clock.getElapsedTime()
    meshRef.current.position.y = position[1] + Math.sin(t + position[0]) * 0.1
    // Color shifting logic
    const hue = (t * 0.05 + (position[0] / 15)) % 1
    const material = meshRef.current.material as THREE.MeshStandardMaterial
    material.color.setHSL(hue, 0.8, 0.6)
  })

  return (
    <Float speed={2} rotationIntensity={0.5} floatIntensity={0.5}>
      <Sphere ref={meshRef} args={[size, 32, 32]} position={position}>
        <MeshDistortMaterial
          color={color}
          speed={3}
          distort={0.2}
          radius={1}
          metalness={0.8}
          roughness={0.2}
          emissive={color}
          emissiveIntensity={0.5}
          transparent
          opacity={0.8}
        />
      </Sphere>
    </Float>
  )
}

const Connection = ({ start, end }: { start: [number, number, number]; end: [number, number, number] }) => {
  const lineRef = useRef<THREE.Line>(null!)
  
  useFrame((state) => {
    const t = state.clock.getElapsedTime()
    const material = lineRef.current.material as THREE.LineBasicMaterial
    material.color.setHSL(hue, 0.8, 0.6)
    material.opacity = 0.2 + Math.sin(t * 2) * 0.1
  })

  return (
    <Line
      ref={lineRef}
      points={[start, end]}
      color="cyan"
      lineWidth={0.5}
      transparent
      opacity={0.3}
    />
  )
}

export function AvogadroMatrix() {
  const nodes = useMemo(() => generateNodes(40), [])
  const connections = useMemo(() => {
    const links = []
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const dist = new THREE.Vector3(...nodes[i].position).distanceTo(new THREE.Vector3(...nodes[j].position))
        if (dist < 5) {
          links.push({ start: nodes[i].position, end: nodes[j].position })
        }
      }
    }
    return links
  }, [nodes])

  return (
    <div className="w-full h-full bg-black/40 rounded-3xl overflow-hidden border border-white/10 backdrop-blur-xl relative group">
      {/* HUD Overlay */}
      <div className="absolute top-6 left-6 z-10 pointer-events-none">
        <h2 className="text-xs font-mono tracking-[0.2em] text-cyan-400 uppercase opacity-50">Memory Resonance</h2>
        <div className="text-2xl font-light text-white mt-1">Avogadro Matrix</div>
        <div className="flex gap-4 mt-4">
          <div className="flex flex-col">
            <span className="text-[10px] font-mono text-white/30 uppercase">Nodes</span>
            <span className="text-sm font-mono text-white/80">6.022e23</span>
          </div>
          <div className="flex flex-col border-l border-white/10 pl-4">
            <span className="text-[10px] font-mono text-white/30 uppercase">Entropy</span>
            <span className="text-sm font-mono text-white/80">0.042 Δ</span>
          </div>
        </div>
      </div>

      <Canvas camera={{ position: [0, 0, 20], fov: 45 }}>
        <color attach="background" args={['#050505']} />
        <fog attach="fog" args={['#050505', 10, 50]} />
        
        <ambientLight intensity={0.2} />
        <pointLight position={[10, 10, 10]} intensity={1} color="#ff00ff" />
        <pointLight position={[-10, -10, -10]} intensity={1} color="#00ffff" />
        
        {nodes.map((node) => (
          <Atom key={node.id} {...node} />
        ))}
        
        {connections.map((link, i) => (
          <Connection key={i} {...link} />
        ))}

        <OrbitControls 
          enablePan={false} 
          autoRotate 
          autoRotateSpeed={0.5}
          maxDistance={30}
          minDistance={10}
        />
      </Canvas>

      {/* Sexy subtle vignette */}
      <div className="absolute inset-0 pointer-events-none bg-[radial-gradient(circle_at_center,transparent_0%,rgba(0,0,0,0.4)_100%)]" />
    </div>
  )
}
