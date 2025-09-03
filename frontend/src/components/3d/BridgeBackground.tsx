import { useRef, useMemo } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { Float, OrbitControls } from '@react-three/drei'
import * as THREE from 'three'

// Particle system component
const ParticleField = () => {
  const meshRef = useRef<THREE.Points>(null)

  const [positions, colors] = useMemo(() => {
    const count = 1000
    const positions = new Float32Array(count * 3)
    const colors = new Float32Array(count * 3)

    for (let i = 0; i < count; i++) {
      // Random positions in a large sphere
      positions[i * 3] = (Math.random() - 0.5) * 20
      positions[i * 3 + 1] = (Math.random() - 0.5) * 20
      positions[i * 3 + 2] = (Math.random() - 0.5) * 20

      // Blue to purple gradient colors
      const t = Math.random()
      colors[i * 3] = 0.3 + t * 0.5     // Red
      colors[i * 3 + 1] = 0.4 + t * 0.3 // Green  
      colors[i * 3 + 2] = 0.8 + t * 0.2 // Blue
    }

    return [positions, colors]
  }, [])

  useFrame((state) => {
    if (meshRef.current) {
      meshRef.current.rotation.x = state.clock.elapsedTime * 0.05
      meshRef.current.rotation.y = state.clock.elapsedTime * 0.1
    }
  })

  return (
    <points ref={meshRef}>
      <bufferGeometry>
        <bufferAttribute
          attach="attributes-position"
          count={positions.length / 3}
          array={positions}
          itemSize={3}
        />
        <bufferAttribute
          attach="attributes-color"
          count={colors.length / 3}
          array={colors}
          itemSize={3}
        />
      </bufferGeometry>
      <pointsMaterial
        size={0.02}
        vertexColors
        transparent
        opacity={0.6}
        sizeAttenuation
      />
    </points>
  )
}

// Floating bridge nodes
const BridgeNode = ({ position, color }: { position: [number, number, number], color: string }) => {
  return (
    <Float speed={2} rotationIntensity={1} floatIntensity={2}>
      <mesh position={position}>
        <sphereGeometry args={[0.1, 16, 16]} />
        <meshStandardMaterial 
          color={color} 
          emissive={color}
          emissiveIntensity={0.2}
          transparent
          opacity={0.8}
        />
      </mesh>
    </Float>
  )
}

// Connection lines between nodes
const ConnectionLine = ({ 
  start, 
  end 
}: { 
  start: [number, number, number], 
  end: [number, number, number] 
}) => {
  const ref = useRef<THREE.Mesh>(null)
  
  useFrame((state) => {
    if (ref.current) {
      const material = ref.current.material as THREE.MeshBasicMaterial
      material.opacity = 0.3 + Math.sin(state.clock.elapsedTime * 2) * 0.2
    }
  })

  const points = [new THREE.Vector3(...start), new THREE.Vector3(...end)]
  const geometry = new THREE.BufferGeometry().setFromPoints(points)

  return (
    <line>
      <bufferGeometry {...geometry} />
      <lineBasicMaterial color="#3b82f6" transparent opacity={0.5} />
    </line>
  )
}

// Main 3D scene
const Scene = () => {
  return (
    <>
      {/* Ambient lighting */}
      <ambientLight intensity={0.2} />
      <pointLight position={[10, 10, 10]} intensity={0.5} color="#3b82f6" />
      <pointLight position={[-10, -10, -10]} intensity={0.3} color="#10b981" />

      {/* Particle field */}
      <ParticleField />

      {/* Bridge nodes representing ICP and Ethereum */}
      <BridgeNode position={[-3, 0, 0]} color="#3b82f6" />
      <BridgeNode position={[3, 0, 0]} color="#10b981" />
      <BridgeNode position={[0, 2, 0]} color="#f59e0b" />

      {/* Connection lines */}
      <ConnectionLine start={[-3, 0, 0]} end={[3, 0, 0]} />
      <ConnectionLine start={[-3, 0, 0]} end={[0, 2, 0]} />
      <ConnectionLine start={[3, 0, 0]} end={[0, 2, 0]} />

      {/* Camera controls */}
      <OrbitControls 
        enableZoom={false}
        enablePan={false}
        autoRotate
        autoRotateSpeed={0.5}
        maxPolarAngle={Math.PI / 2}
        minPolarAngle={Math.PI / 2}
      />
    </>
  )
}

const BridgeBackground = () => {
  return (
    <div className="w-full h-full">
      <Canvas
        camera={{ position: [0, 0, 8], fov: 60 }}
        style={{ background: 'transparent' }}
      >
        <Scene />
      </Canvas>
    </div>
  )
}

export default BridgeBackground
