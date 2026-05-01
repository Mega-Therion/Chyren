'use client';

import { useRef, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import type { BrainState } from './NeuralBrain';

const vertexShader = /* glsl */ `
  uniform float uTime;
  uniform float uAudioLevel;
  varying vec3 vNormal;
  varying vec3 vPosition;
  varying float vDisplace;

  // Simplex-style noise helpers
  vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
  vec4 mod289(vec4 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
  vec4 permute(vec4 x) { return mod289(((x*34.0)+10.0)*x); }
  vec4 taylorInvSqrt(vec4 r) { return 1.79284291400159 - 0.85373472095314 * r; }

  float snoise(vec3 v) {
    const vec2 C = vec2(1.0/6.0, 1.0/3.0);
    const vec4 D = vec4(0.0, 0.5, 1.0, 2.0);
    vec3 i  = floor(v + dot(v, C.yyy));
    vec3 x0 = v - i + dot(i, C.xxx);
    vec3 g  = step(x0.yzx, x0.xyz);
    vec3 l  = 1.0 - g;
    vec3 i1 = min( g.xyz, l.zxy );
    vec3 i2 = max( g.xyz, l.zxy );
    vec3 x1 = x0 - i1 + C.xxx;
    vec3 x2 = x0 - i2 + C.yyy;
    vec3 x3 = x0 - D.yyy;
    i = mod289(i);
    vec4 p = permute( permute( permute(
               i.z + vec4(0.0, i1.z, i2.z, 1.0))
             + i.y + vec4(0.0, i1.y, i2.y, 1.0))
             + i.x + vec4(0.0, i1.x, i2.x, 1.0));
    float n_ = 0.142857142857;
    vec3 ns = n_ * D.wyz - D.xzx;
    vec4 j = p - 49.0 * floor(p * ns.z * ns.z);
    vec4 x_ = floor(j * ns.z);
    vec4 y_ = floor(j - 7.0 * x_);
    vec4 x = x_ *ns.x + ns.yyyy;
    vec4 y = y_ *ns.x + ns.yyyy;
    vec4 h = 1.0 - abs(x) - abs(y);
    vec4 b0 = vec4( x.xy, y.xy );
    vec4 b1 = vec4( x.zw, y.zw );
    vec4 s0 = floor(b0)*2.0 + 1.0;
    vec4 s1 = floor(b1)*2.0 + 1.0;
    vec4 sh = -step(h, vec4(0.0));
    vec4 a0 = b0.xzyw + s0.xzyw*sh.xxyy;
    vec4 a1 = b1.xzyw + s1.xzyw*sh.zzww;
    vec3 p0 = vec3(a0.xy, h.x);
    vec3 p1 = vec3(a0.zw, h.y);
    vec3 p2 = vec3(a1.xy, h.z);
    vec3 p3 = vec3(a1.zw, h.w);
    vec4 norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2,p2), dot(p3,p3)));
    p0 *= norm.x; p1 *= norm.y; p2 *= norm.z; p3 *= norm.w;
    vec4 m = max(0.6 - vec4(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), 0.0);
    m = m * m;
    return 42.0 * dot( m*m, vec4( dot(p0,x0), dot(p1,x1), dot(p2,x2), dot(p3,x3)));
  }

  void main() {
    vNormal = normal;
    float boost = 1.0 + uAudioLevel * 2.0;
    float n1 = snoise(position * 1.8 + uTime * 0.18 * boost);
    float n2 = snoise(position * 3.2 - uTime * 0.12 * boost);
    float n3 = snoise(position * 5.5 + uTime * 0.09 * boost);
    float displacement = (n1 * 0.35 + n2 * 0.18 + n3 * 0.08) * (1.0 + uAudioLevel * 1.5);
    vDisplace = displacement;
    vec3 displaced = position + normal * displacement;
    vPosition = displaced;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(displaced, 1.0);
  }
`;

const fragmentShader = /* glsl */ `
  uniform float uTime;
  uniform vec3 uColorA;
  uniform vec3 uColorB;
  uniform vec3 uColorC;
  uniform float uAudioLevel;
  varying vec3 vNormal;
  varying vec3 vPosition;
  varying float vDisplace;

  void main() {
    vec3 viewDir = normalize(cameraPosition - vPosition);
    float fresnel = pow(1.0 - max(dot(vNormal, viewDir), 0.0), 3.0);
    float t = vDisplace * 2.0 + 0.5;
    vec3 col = mix(uColorA, uColorB, clamp(t, 0.0, 1.0));
    col = mix(col, uColorC, clamp(t - 1.0, 0.0, 1.0));
    // Rim glow
    col += uColorB * fresnel * (1.8 + uAudioLevel * 2.5);
    // Pulsing core brightness
    float pulse = sin(uTime * 2.0) * 0.5 + 0.5;
    col += uColorA * 0.15 * pulse;
    gl_FragColor = vec4(col, 0.92);
  }
`;

const STATE_COLORS: Record<BrainState, [string, string, string]> = {
  idle:      ['#f59e0b', '#b45309', '#fbbf24'],
  listening: ['#bc13fe', '#6d28d9', '#e879f9'],
  thinking:  ['#ff2d75', '#9f1239', '#fb7185'],
  speaking:  ['#00f2ff', '#0891b2', '#67e8f9'],
};

function hexToVec3(hex: string): THREE.Vector3 {
  const r = parseInt(hex.slice(1, 3), 16) / 255;
  const g = parseInt(hex.slice(3, 5), 16) / 255;
  const b = parseInt(hex.slice(5, 7), 16) / 255;
  return new THREE.Vector3(r, g, b);
}

export function SovereignSphere({ state, audioLevel }: { state: BrainState; audioLevel: number }) {
  const meshRef = useRef<THREE.Mesh>(null);
  const matRef = useRef<THREE.ShaderMaterial>(null);

  const uniforms = useMemo(() => ({
    uTime:       { value: 0 },
    uAudioLevel: { value: 0 },
    uColorA:     { value: hexToVec3(STATE_COLORS.idle[0]) },
    uColorB:     { value: hexToVec3(STATE_COLORS.idle[1]) },
    uColorC:     { value: hexToVec3(STATE_COLORS.idle[2]) },
  }), []);

  const targetColors = useRef({ a: new THREE.Vector3(), b: new THREE.Vector3(), c: new THREE.Vector3() });

  useFrame(({ clock }) => {
    if (!matRef.current) return;
    const u = matRef.current.uniforms;
    u.uTime.value = clock.getElapsedTime();
    u.uAudioLevel.value += (audioLevel - u.uAudioLevel.value) * 0.1;

    const [ca, cb, cc] = STATE_COLORS[state];
    targetColors.current.a.copy(hexToVec3(ca));
    targetColors.current.b.copy(hexToVec3(cb));
    targetColors.current.c.copy(hexToVec3(cc));

    u.uColorA.value.lerp(targetColors.current.a, 0.04);
    u.uColorB.value.lerp(targetColors.current.b, 0.04);
    u.uColorC.value.lerp(targetColors.current.c, 0.04);

    if (meshRef.current) {
      meshRef.current.rotation.y += 0.003;
      meshRef.current.rotation.x += 0.001;
    }
  });

  return (
    <mesh ref={meshRef}>
      <icosahedronGeometry args={[1.4, 64]} />
      <shaderMaterial
        ref={matRef}
        vertexShader={vertexShader}
        fragmentShader={fragmentShader}
        uniforms={uniforms}
        transparent
        side={THREE.FrontSide}
      />
    </mesh>
  );
}
