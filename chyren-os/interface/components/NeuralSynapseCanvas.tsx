'use client';

import React, { useEffect, useRef } from 'react';

export const NeuralSynapseCanvas = ({ isActive }: { isActive: boolean }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let particles: { x: number; y: number; vx: number; vy: number; life: number }[] = [];
    const width = canvas.width;
    const height = canvas.height;

    function animate() {
      if (!ctx) return;
      ctx.fillStyle = 'rgba(3, 7, 18, 0.15)';
      ctx.fillRect(0, 0, width, height);

      if (isActive && Math.random() > 0.8) {
        particles.push({
          x: Math.random() * width,
          y: Math.random() * height,
          vx: (Math.random() - 0.5) * 2,
          vy: (Math.random() - 0.5) * 2,
          life: 1.0,
        });
      }

      particles = particles.filter(p => p.life > 0);
      particles.forEach(p => {
        p.x += p.vx;
        p.y += p.vy;
        p.life -= 0.01;
        ctx.fillStyle = `rgba(99, 102, 241, ${p.life})`;
        ctx.beginPath();
        ctx.arc(p.x, p.y, 1.5, 0, Math.PI * 2);
        ctx.fill();
      });

      requestAnimationFrame(animate);
    }

    animate();
  }, [isActive]);

  return <canvas ref={canvasRef} className="absolute inset-0 w-full h-full opacity-60" />;
};
