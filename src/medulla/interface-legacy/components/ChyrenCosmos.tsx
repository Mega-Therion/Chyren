'use client';

import { useRef, useEffect } from 'react';
import { Canvas, useFrame, useThree } from '@react-three/fiber';
import { EffectComposer, Bloom, Vignette } from '@react-three/postprocessing';
import type * as THREE from 'three';
import { SovereignSphere } from './SovereignSphere';
import { ParticleCosmos } from './ParticleCosmos';
import type { BrainState } from './NeuralBrain';

// Cinematic camera controller — intro push + breathing + mouse parallax
function CameraController({ state }: { state: BrainState }) {
  const { camera } = useThree();
  const mouse = useRef({ x: 0, y: 0 });
  const introProgress = useRef(0);

  useEffect(() => {
    const onMove = (e: MouseEvent) => {
      mouse.current.x = (e.clientX / window.innerWidth  - 0.5) * 2;
      mouse.current.y = (e.clientY / window.innerHeight - 0.5) * 2;
    };
    window.addEventListener('mousemove', onMove);
    return () => window.removeEventListener('mousemove', onMove);
  }, []);

  useFrame(({ clock }) => {
    const t = clock.getElapsedTime();
    // Cinematic intro: camera eases from z=18 → z=7 over first 4 seconds
    introProgress.current = Math.min(1, t / 4.0);
    const ease = 1 - Math.pow(1 - introProgress.current, 3);
    const baseZ = 18 - (18 - 7) * ease;

    // Breathing oscillation (subtle)
    const breathe = Math.sin(t * 0.4) * 0.08;

    // State-driven zoom: thinking pulls camera slightly closer
    const stateZ = state === 'thinking' ? -0.3 : state === 'speaking' ? -0.5 : 0;

    camera.position.z = baseZ + breathe + stateZ;
    camera.position.x += (mouse.current.x * 0.6 - camera.position.x) * 0.03;
    camera.position.y += (-mouse.current.y * 0.4 - camera.position.y) * 0.03;
    camera.lookAt(0, 0, 0);
  });

  return null;
}

// Outer ring pulse ring (wireframe torus)
function SovereignRing({ state }: { state: BrainState }) {
  const meshRef = useRef<THREE.Mesh<THREE.TorusGeometry, THREE.MeshBasicMaterial>>(null);
  const COLOR_MAP: Record<BrainState, string> = {
    idle: '#333333', listening: '#bc13fe', thinking: '#ff0080', speaking: '#00f2ff',
  };

  useFrame(({ clock }) => {
    if (!meshRef.current) return;
    const t = clock.getElapsedTime();
    meshRef.current.rotation.x = t * 0.12;
    meshRef.current.rotation.z = t * 0.07;
    const scale = 1 + Math.sin(t * 1.5) * 0.02;
    meshRef.current.scale.setScalar(scale);
    meshRef.current.material.color.set(COLOR_MAP[state]);
  });

  return (
    <mesh ref={meshRef}>
      <torusGeometry args={[2.1, 0.002, 4, 128]} />
      <meshBasicMaterial color="#ffffff" transparent opacity={0.1} />
    </mesh>
  );
}

function SovereignRing2({ state }: { state: BrainState }) {
  const meshRef = useRef<THREE.Mesh<THREE.TorusGeometry, THREE.MeshBasicMaterial>>(null);
  const COLOR_MAP: Record<BrainState, string> = {
    idle: '#222222', listening: '#6d28d9', thinking: '#ff0080', speaking: '#0891b2',
  };

  useFrame(({ clock }) => {
    if (!meshRef.current) return;
    const t = clock.getElapsedTime();
    meshRef.current.rotation.x = -t * 0.08;
    meshRef.current.rotation.y = t * 0.11;
    const scale = 1 + Math.sin(t * 1.1 + 1.0) * 0.025;
    meshRef.current.scale.setScalar(scale);
    meshRef.current.material.color.set(COLOR_MAP[state]);
  });

  return (
    <mesh ref={meshRef}>
      <torusGeometry args={[2.4, 0.001, 4, 96]} />
      <meshBasicMaterial color="#ffffff" transparent opacity={0.05} />
    </mesh>
  );
}

// Post-processing effect set — state-reactive bloom intensity
function PostFX({ state, audioLevel }: { state: BrainState; audioLevel: number }) {
  const bloomRef = useRef<{ intensity: number }>(null);
  const caRef = useRef<{ offset: THREE.Vector2 }>(null);

  const BLOOM_MAP: Record<BrainState, number> = {
    idle: 0.2, listening: 0.8, thinking: 1.0, speaking: 1.2,
  };

  useFrame(() => {
    const target = BLOOM_MAP[state] + audioLevel * 1.0;
    if (bloomRef.current) {
      bloomRef.current.intensity += (target - bloomRef.current.intensity) * 0.05;
    }
    if (caRef.current) {
      const strength = 0.0002 + audioLevel * 0.001;
      caRef.current.offset.set(strength, strength);
    }
  });

  return (
    <EffectComposer>
      <Bloom
        ref={bloomRef}
        intensity={0.2}
        luminanceThreshold={0.4}
        luminanceSmoothing={0.9}
        mipmapBlur
      />
      <Vignette eskil={false} offset={0.3} darkness={0.95} />
    </EffectComposer>
  );
}

interface ChyrenCosmosProps {
  state: BrainState;
  audioLevel: number;
}

export function ChyrenCosmos({ state, audioLevel }: ChyrenCosmosProps) {
  return (
    <Canvas
      camera={{ position: [0, 0, 18], fov: 60 }}
      dpr={[1, 2]}
      gl={{ antialias: true, alpha: false, powerPreference: 'high-performance' }}
      style={{ background: '#000000' }}
    >
      <ambientLight intensity={0.02} />
      <CameraController state={state} />
      <ParticleCosmos state={state} audioLevel={audioLevel} />
      <SovereignSphere state={state} audioLevel={audioLevel} />
      <SovereignRing state={state} />
      <SovereignRing2 state={state} />
      <PostFX state={state} audioLevel={audioLevel} />
    </Canvas>
  );
}
