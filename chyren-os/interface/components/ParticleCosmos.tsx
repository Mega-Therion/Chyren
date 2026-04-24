'use client';

import { useRef, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import type { BrainState } from './NeuralBrain';

const PARTICLE_COUNT = 4800;

const STATE_PARTICLE_COLORS: Record<BrainState, string> = {
  idle:      '#f59e0b',
  listening: '#bc13fe',
  thinking:  '#ff2d75',
  speaking:  '#00f2ff',
};

const particleVertexShader = /* glsl */ `
  attribute float aScale;
  attribute float aPhase;
  uniform float uTime;
  uniform float uAudioLevel;
  varying float vOpacity;

  void main() {
    float pulse = sin(uTime * 1.2 + aPhase) * 0.5 + 0.5;
    vOpacity = 0.25 + pulse * 0.55;
    vec4 mv = modelViewMatrix * vec4(position, 1.0);
    float size = aScale * (1.0 + uAudioLevel * 2.0 + pulse * 0.4);
    gl_PointSize = size * (280.0 / -mv.z);
    gl_Position = projectionMatrix * mv;
  }
`;

const particleFragmentShader = /* glsl */ `
  uniform vec3 uColor;
  varying float vOpacity;

  void main() {
    float d = distance(gl_PointCoord, vec2(0.5));
    if (d > 0.5) discard;
    float alpha = (1.0 - d * 2.0);
    alpha = alpha * alpha * vOpacity;
    gl_FragColor = vec4(uColor, alpha);
  }
`;

export function ParticleCosmos({ state, audioLevel }: { state: BrainState; audioLevel: number }) {
  const pointsRef = useRef<THREE.Points>(null);
  const matRef = useRef<THREE.ShaderMaterial>(null);
  const targetColor = useRef(new THREE.Color(STATE_PARTICLE_COLORS.idle));

  const { positions, scales, phases } = useMemo(() => {
    const positions = new Float32Array(PARTICLE_COUNT * 3);
    const scales    = new Float32Array(PARTICLE_COUNT);
    const phases    = new Float32Array(PARTICLE_COUNT);

    for (let i = 0; i < PARTICLE_COUNT; i++) {
      // Galaxy spiral arms + halo distribution
      const layer = Math.random();
      if (layer < 0.6) {
        // Spiral arms
        const arm = Math.floor(Math.random() * 3);
        const armAngle = (arm / 3) * Math.PI * 2;
        const r = 2.5 + Math.random() * 9.0;
        const theta = armAngle + r * 0.38 + (Math.random() - 0.5) * 0.7;
        positions[i * 3]     = Math.cos(theta) * r;
        positions[i * 3 + 1] = (Math.random() - 0.5) * 1.2;
        positions[i * 3 + 2] = Math.sin(theta) * r;
        scales[i] = 0.8 + Math.random() * 1.8;
      } else if (layer < 0.85) {
        // Halo sphere
        const phi   = Math.acos(2 * Math.random() - 1);
        const theta = Math.random() * Math.PI * 2;
        const r     = 10 + Math.random() * 6;
        positions[i * 3]     = r * Math.sin(phi) * Math.cos(theta);
        positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta);
        positions[i * 3 + 2] = r * Math.cos(phi);
        scales[i] = 0.4 + Math.random() * 0.8;
      } else {
        // Dense core
        const phi   = Math.acos(2 * Math.random() - 1);
        const theta = Math.random() * Math.PI * 2;
        const r     = Math.random() * 2.8;
        positions[i * 3]     = r * Math.sin(phi) * Math.cos(theta);
        positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta) * 0.4;
        positions[i * 3 + 2] = r * Math.cos(phi);
        scales[i] = 1.2 + Math.random() * 2.0;
      }
      phases[i] = Math.random() * Math.PI * 2;
    }
    return { positions, scales, phases };
  }, []);

  const uniforms = useMemo(() => ({
    uTime:       { value: 0 },
    uAudioLevel: { value: 0 },
    uColor:      { value: new THREE.Color(STATE_PARTICLE_COLORS.idle) },
  }), []);

  useFrame(({ clock }) => {
    if (!matRef.current) return;
    const u = matRef.current.uniforms;
    u.uTime.value = clock.getElapsedTime();
    u.uAudioLevel.value += (audioLevel - u.uAudioLevel.value) * 0.08;

    targetColor.current.set(STATE_PARTICLE_COLORS[state]);
    u.uColor.value.lerp(targetColor.current, 0.03);

    if (pointsRef.current) {
      pointsRef.current.rotation.y += 0.00035 + audioLevel * 0.001;
    }
  });

  return (
    <points ref={pointsRef}>
      <bufferGeometry>
        <bufferAttribute attach="attributes-position" args={[positions, 3]} />
        <bufferAttribute attach="attributes-aScale"   args={[scales, 1]} />
        <bufferAttribute attach="attributes-aPhase"   args={[phases, 1]} />
      </bufferGeometry>
      <shaderMaterial
        ref={matRef}
        vertexShader={particleVertexShader}
        fragmentShader={particleFragmentShader}
        uniforms={uniforms}
        transparent
        depthWrite={false}
        blending={THREE.AdditiveBlending}
      />
    </points>
  );
}
