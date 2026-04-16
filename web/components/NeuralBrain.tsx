'use client';

import { useEffect, useRef } from 'react';

interface Node {
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  pulsePhase: number;
  pulseSpeed: number;
  connections: number[];
}

interface Spark {
  fromNode: number;
  toNode: number;
  progress: number;
  speed: number;
  startedAt: number;
}

export type BrainState = 'idle' | 'listening' | 'thinking' | 'speaking';

export function NeuralBrain({
  _isActive = false,
  audioLevel = 0,
  state = 'idle',
}: {
  _isActive?: boolean;
  audioLevel?: number;
  state?: BrainState;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animRef = useRef<number>(0);
  const stateRef = useRef<{ nodes: Node[]; sparks: Spark[]; t: number }>({
    nodes: [],
    sparks: [],
    t: 0,
  });

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const resize = () => {
      canvas.width = canvas.offsetWidth;
      canvas.height = canvas.offsetHeight;
    };
    resize();
    window.addEventListener('resize', resize);

    // Build nodes
    const NODE_COUNT = 22;
    const nodes: Node[] = [];
    for (let i = 0; i < NODE_COUNT; i++) {
      nodes.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        vx: (Math.random() - 0.5) * 0.3,
        vy: (Math.random() - 0.5) * 0.3,
        radius: 2 + Math.random() * 3,
        pulsePhase: Math.random() * Math.PI * 2,
        pulseSpeed: 0.02 + Math.random() * 0.03,
        connections: [],
      });
    }

    // Connect nearby nodes
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const dx = nodes[i].x - nodes[j].x;
        const dy = nodes[i].y - nodes[j].y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist < 220) {
          nodes[i].connections.push(j);
          nodes[j].connections.push(i);
        }
      }
    }

    stateRef.current.nodes = nodes;
    stateRef.current.sparks = [];

    // Dynamic colors based on user request:
    // Blue for speaking (responding)
    // Pink for thinking (processing)
    // Purple for listening
    const getBaseColor = (s: BrainState, a: number) => {
      switch (s) {
        case 'speaking': return `rgba(0, 242, 255, ${a})`; // Cyan/Blue
        case 'thinking': return `rgba(255, 45, 117, ${a})`; // Rose/Pink
        case 'listening': return `rgba(188, 19, 254, ${a})`; // Violet/Purple
        default: return `rgba(245, 158, 11, ${a})`; // Amber/Idle
      }
    };

    // Bridge: read audioLevel from dataset each frame to avoid closure staleness
    const readAudioLevel = () => parseFloat(canvas.dataset.audioLevel ?? '0') || 0;
    const readState = () => (canvas.dataset.brainState as BrainState) || 'idle';

    const spawnSpark = () => {
      const { nodes, sparks } = stateRef.current;
      if (sparks.length >= 6) return;
      const fromIdx = Math.floor(Math.random() * nodes.length);
      const n = nodes[fromIdx];
      if (!n.connections.length) return;
      const toIdx = n.connections[Math.floor(Math.random() * n.connections.length)];
      sparks.push({ fromNode: fromIdx, toNode: toIdx, progress: 0, speed: 0.007 + Math.random() * 0.006, startedAt: Date.now() });
    };

    let lastSpawn = 0;

    const draw = () => {
      const { nodes, sparks } = stateRef.current;
      const W = canvas.width;
      const H = canvas.height;
      stateRef.current.t += 0.016;

      const al = readAudioLevel();
      const s = readState();

      ctx.clearRect(0, 0, W, H);

      const boost = 1 + al * 1.4;
      nodes.forEach(n => {
        n.x += n.vx * boost;
        n.y += n.vy * boost;
        if (n.x < 0 || n.x > W) { n.vx *= -1; n.x = Math.max(0, Math.min(W, n.x)); }
        if (n.y < 0 || n.y > H) { n.vy *= -1; n.y = Math.max(0, Math.min(H, n.y)); }
        n.pulsePhase += n.pulseSpeed;
      });

      const edgeBoost = 0.18 + al * 0.22;
      for (let i = 0; i < nodes.length; i++) {
        const a = nodes[i];
        for (const j of a.connections) {
          if (j <= i) continue;
          const b = nodes[j];
          const dx = b.x - a.x;
          const dy = b.y - a.y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          const alpha = Math.max(0, (1 - dist / 220) * edgeBoost);
          ctx.strokeStyle = getBaseColor(s, alpha);
          ctx.lineWidth = 0.8 + al * 0.6;
          ctx.beginPath();
          ctx.moveTo(a.x, a.y);
          ctx.lineTo(b.x, b.y);
          ctx.stroke();
        }
      }

      nodes.forEach(n => {
        const pulse = Math.sin(n.pulsePhase) * 0.5 + 0.5;
        const r = n.radius + pulse * 1.5;
        const alpha = 0.4 + pulse * 0.4;

        const grd = ctx.createRadialGradient(n.x, n.y, 0, n.x, n.y, r * 3);
        grd.addColorStop(0, getBaseColor(s, alpha * 0.6));
        grd.addColorStop(1, getBaseColor(s, 0));
        ctx.fillStyle = grd;
        ctx.beginPath();
        ctx.arc(n.x, n.y, r * 3, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = getBaseColor(s, alpha);
        ctx.beginPath();
        ctx.arc(n.x, n.y, r, 0, Math.PI * 2);
        ctx.fill();
      });

      const spawnInterval = s !== 'idle'
        ? Math.max(80, 400 - al * 300)
        : Math.max(400, 1800 - al * 1400);
      if (Date.now() - lastSpawn > spawnInterval) {
        spawnSpark();
        lastSpawn = Date.now();
      }

      stateRef.current.sparks = sparks.filter(s => s.progress < 1);
      stateRef.current.sparks.forEach(spk => {
        spk.progress = Math.min(1, spk.progress + spk.speed);
        const from = nodes[spk.fromNode];
        const to = nodes[spk.toNode];
        const p = spk.progress;

        const midFactor = Math.sin(p * Math.PI);
        const sparkR = 1.5 + midFactor * 3.5;
        const opacity = 0.3 + midFactor * 0.7;

        const sx = from.x + (to.x - from.x) * p;
        const sy = from.y + (to.y - from.y) * p;

        const sgrd = ctx.createRadialGradient(sx, sy, 0, sx, sy, sparkR * 4);
        sgrd.addColorStop(0, getBaseColor(s, opacity * 0.8));
        sgrd.addColorStop(0.5, getBaseColor(s, opacity * 0.3));
        sgrd.addColorStop(1, getBaseColor(s, 0));
        ctx.fillStyle = sgrd;
        ctx.beginPath();
        ctx.arc(sx, sy, sparkR * 4, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = getBaseColor(s, opacity);
        ctx.beginPath();
        ctx.arc(sx, sy, sparkR, 0, Math.PI * 2);
        ctx.fill();
      });

      animRef.current = requestAnimationFrame(draw);
    };

    animRef.current = requestAnimationFrame(draw);

    return () => {
      window.removeEventListener('resize', resize);
      cancelAnimationFrame(animRef.current);
    };
  }, []); // Only once

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    canvas.dataset.audioLevel = String(audioLevel);
    canvas.dataset.brainState = state;
  }, [audioLevel, state]);


  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full"
      style={{ opacity: 0.65 }}
    />
  );
}
