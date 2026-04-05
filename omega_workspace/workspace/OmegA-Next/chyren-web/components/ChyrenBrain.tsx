'use client'

import { useEffect, useRef } from 'react'
import * as THREE from 'three'

export interface BrainState {
  adccl: number        // 0-1 verification gate
  provider: number     // 0-1 active spoke
  threat: number       // 0-1 threat fabric
  phylactery: number   // 0-1 identity/memory
  ledger: number       // 0-1 ledger write
  alignment: number    // 0-1 alignment layer
}

export const IDLE_BRAIN: BrainState = {
  adccl: 0.05,
  provider: 0.05,
  threat: 0.02,
  phylactery: 0.08,
  ledger: 0.02,
  alignment: 0.05,
}

// Module definitions: name, position on sphere, color
const MODULES = [
  { key: 'adccl',      label: 'ADCCL',       phi: 0.3,  theta: 0.0,  color: new THREE.Color(0x00e5ff) },  // cyan - top front
  { key: 'alignment',  label: 'ALIGNMENT',   phi: 0.5,  theta: 0.4,  color: new THREE.Color(0x7c4dff) },  // purple - upper side
  { key: 'provider',   label: 'PROVIDER',    phi: 1.0,  theta: -0.5, color: new THREE.Color(0x00e676) },  // green - mid left
  { key: 'phylactery', label: 'PHYLACTERY',  phi: 1.2,  theta: 0.8,  color: new THREE.Color(0xffd740) },  // gold - mid right
  { key: 'ledger',     label: 'LEDGER',      phi: 1.6,  theta: 0.2,  color: new THREE.Color(0xff6d00) },  // orange - lower front
  { key: 'threat',     label: 'THREAT',      phi: 0.8,  theta: 2.8,  color: new THREE.Color(0xff1744) },  // red - back
] as const

type ModuleKey = typeof MODULES[number]['key']

function spherePos(phi: number, theta: number, r: number): THREE.Vector3 {
  return new THREE.Vector3(
    r * Math.sin(phi) * Math.cos(theta),
    r * Math.cos(phi),
    r * Math.sin(phi) * Math.sin(theta)
  )
}

export default function ChyrenBrain({ state = IDLE_BRAIN }: { state?: BrainState }) {
  const mountRef = useRef<HTMLDivElement>(null)
  const stateRef = useRef(state)
  stateRef.current = state

  useEffect(() => {
    const el = mountRef.current
    if (!el) return

    const w = el.clientWidth
    const h = el.clientHeight

    // Scene
    const scene = new THREE.Scene()
    const camera = new THREE.PerspectiveCamera(45, w / h, 0.1, 100)
    camera.position.set(0, 0, 4.5)

    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
    renderer.setSize(w, h)
    renderer.setPixelRatio(window.devicePixelRatio)
    el.appendChild(renderer.domElement)

    // Brain sphere (wireframe)
    const brainGeo = new THREE.SphereGeometry(1.5, 32, 24)
    const brainMat = new THREE.MeshBasicMaterial({
      color: 0x1a2744,
      wireframe: true,
      transparent: true,
      opacity: 0.15,
    })
    const brainMesh = new THREE.Mesh(brainGeo, brainMat)
    scene.add(brainMesh)

    // Build nodes
    const nodes: {
      key: ModuleKey
      mesh: THREE.Mesh
      glow: THREE.Mesh
      mat: THREE.MeshBasicMaterial
      glowMat: THREE.MeshBasicMaterial
      baseColor: THREE.Color
      pos: THREE.Vector3
    }[] = []

    for (const mod of MODULES) {
      const pos = spherePos(mod.phi, mod.theta, 1.5)

      // Core sphere
      const geo = new THREE.SphereGeometry(0.08, 16, 16)
      const mat = new THREE.MeshBasicMaterial({ color: mod.color.clone() })
      const mesh = new THREE.Mesh(geo, mat)
      mesh.position.copy(pos)
      scene.add(mesh)

      // Glow halo
      const glowGeo = new THREE.SphereGeometry(0.18, 16, 16)
      const glowMat = new THREE.MeshBasicMaterial({
        color: mod.color.clone(),
        transparent: true,
        opacity: 0.0,
      })
      const glow = new THREE.Mesh(glowGeo, glowMat)
      glow.position.copy(pos)
      scene.add(glow)

      nodes.push({ key: mod.key, mesh, glow, mat, glowMat, baseColor: mod.color.clone(), pos })
    }

    // Neural pathways (lines between nodes)
    const lineMat = new THREE.LineBasicMaterial({ color: 0x1e3a5f, transparent: true, opacity: 0.3 })
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const pts = [nodes[i].pos.clone(), nodes[j].pos.clone()]
        const lineGeo = new THREE.BufferGeometry().setFromPoints(pts)
        scene.add(new THREE.Line(lineGeo, lineMat))
      }
    }

    // Pulse rings per node
    const rings: { mesh: THREE.Mesh; mat: THREE.MeshBasicMaterial; nodeIdx: number; phase: number }[] = []
    for (let i = 0; i < nodes.length; i++) {
      const ringGeo = new THREE.RingGeometry(0.1, 0.14, 32)
      const ringMat = new THREE.MeshBasicMaterial({
        color: nodes[i].baseColor.clone(),
        transparent: true,
        opacity: 0,
        side: THREE.DoubleSide,
      })
      const ring = new THREE.Mesh(ringGeo, ringMat)
      ring.position.copy(nodes[i].pos)
      ring.lookAt(camera.position)
      scene.add(ring)
      rings.push({ mesh: ring, mat: ringMat, nodeIdx: i, phase: Math.random() * Math.PI * 2 })
    }

    // Slow rotation
    let frameId: number
    let t = 0

    function animate() {
      frameId = requestAnimationFrame(animate)
      t += 0.008

      brainMesh.rotation.y = t * 0.3
      brainMesh.rotation.x = Math.sin(t * 0.1) * 0.05

      const s = stateRef.current

      nodes.forEach((node, i) => {
        const activity = s[node.key] ?? 0.05
        const pulse = 0.7 + 0.3 * Math.sin(t * (2 + i * 0.4) + i)
        const intensity = activity * pulse

        // Brightness
        const bright = new THREE.Color()
        bright.copy(node.baseColor).multiplyScalar(0.3 + intensity * 2.5)
        node.mat.color = bright

        // Glow opacity
        node.glowMat.opacity = intensity * 0.5

        // Scale glow with activity
        const scale = 1 + activity * 1.5
        node.glow.scale.setScalar(scale)
      })

      // Pulse rings
      rings.forEach((r) => {
        const activity = s[nodes[r.nodeIdx].key] ?? 0.05
        if (activity > 0.15) {
          r.phase += 0.04
          const p = (r.phase % (Math.PI * 2)) / (Math.PI * 2)
          const ringScale = 1 + p * 3
          r.mesh.scale.setScalar(ringScale)
          r.mat.opacity = (1 - p) * activity * 0.6
          r.mesh.lookAt(camera.position)
        } else {
          r.mat.opacity = 0
        }
      })

      renderer.render(scene, camera)
    }

    animate()

    const onResize = () => {
      const nw = el.clientWidth
      const nh = el.clientHeight
      camera.aspect = nw / nh
      camera.updateProjectionMatrix()
      renderer.setSize(nw, nh)
    }
    window.addEventListener('resize', onResize)

    return () => {
      cancelAnimationFrame(frameId)
      window.removeEventListener('resize', onResize)
      renderer.dispose()
      el.removeChild(renderer.domElement)
    }
  }, [])

  return (
    <div className="relative w-full h-full">
      <div ref={mountRef} className="w-full h-full" />
      {/* Module labels */}
      <div className="absolute bottom-2 left-2 right-2 flex flex-wrap gap-1 justify-center">
        {MODULES.map((mod) => {
          const activity = state[mod.key] ?? 0
          const hex = '#' + mod.color.getHexString()
          return (
            <div
              key={mod.key}
              className="flex items-center gap-1 text-xs px-1.5 py-0.5 rounded"
              style={{
                backgroundColor: `${hex}18`,
                border: `1px solid ${hex}${activity > 0.3 ? 'aa' : '33'}`,
                color: activity > 0.2 ? hex : '#4a6080',
                transition: 'all 0.3s',
              }}
            >
              <span
                className="w-1.5 h-1.5 rounded-full inline-block"
                style={{
                  backgroundColor: hex,
                  opacity: 0.2 + activity * 0.8,
                  boxShadow: activity > 0.3 ? `0 0 4px ${hex}` : 'none',
                  transition: 'all 0.3s',
                }}
              />
              {mod.label}
            </div>
          )
        })}
      </div>
    </div>
  )
}
