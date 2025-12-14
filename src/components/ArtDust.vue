<script setup lang="ts">
import type { Texture } from 'pixi.js'
import { Application, Container, Graphics, Sprite } from 'pixi.js'
import { createNoise3D } from 'simplex-noise'

const el = useTemplateRef('el')

let w = window.innerWidth
let h = window.innerHeight

const SCALE = 150
const WAVE_HEIGHT = 25
const SPACING = 10
const COLORS = [0xE0E0E0, 0xE3E3E3, 0xE6E6E6, 0xDDDDDD]
const CURSOR_INFLUENCE = 25 // Radius of cursor influence
const CURSOR_PULL_RADIUS = -25 // Radius where particles are pulled toward cursor

const noise3d = createNoise3D()

const existingPoints = new Set<string>()
const points: { x: number, y: number, sprite: Sprite, phase: number, speedMod: number }[] = []

// Cursor position
const cursor = { x: w / 2, y: h / 2 }

function getNoiseValue(x: number, y: number, z: number) {
  return noise3d(x / SCALE, y / SCALE, z)
}

const mountedScope = effectScope()

function createStarTexture(app: Application, color: number) {
  const g = new Graphics().star(0, 0, 5, 2, 3).fill(color)
  return app.renderer.generateTexture(g)
}

function addPoints({ textures, container }: { textures: Texture[], container: Container }) {
  for (let x = -SPACING; x < w + SPACING; x += SPACING) {
    for (let y = -SPACING; y < h + SPACING; y += SPACING) {
      const id = `${x}-${y}`
      if (existingPoints.has(id))
        continue
      existingPoints.add(id)

      const colorIndex = Math.floor(Math.random() * COLORS.length)
      const sprite = new Sprite(textures[colorIndex])
      sprite.anchor.set(0.5)
      sprite.scale.set(0.3)
      container.addChild(sprite)

      const phase = Math.random() * Math.PI * 2
      const speedMod = Math.random() * 0.5 + 0.75

      points.push({ x, y, sprite, phase, speedMod })
    }
  }
}

async function setup() {
  if (el.value == null)
    return
  const app = new Application()
  await app.init({
    background: 0xFFFFFF,
    antialias: true,
    resolution: window.devicePixelRatio,
    resizeTo: el.value,
    eventMode: 'none',
    autoDensity: true,
  })
  el.value.appendChild(app.canvas)

  const container = new Container()
  app.stage.addChild(container)

  const textures = COLORS.map(color => createStarTexture(app, color))
  addPoints({ textures, container })

  let time = 0
  app.ticker.add(() => {
    time += 0.005

    for (const p of points) {
      const { x, y, sprite, phase, speedMod } = p

      // Create flowing wave motion with noise
      const noiseVal = getNoiseValue(x, y, time * speedMod)

      // Vertical wave displacement
      const verticalOffset = Math.sin(x / 100 + time * 2 * speedMod + phase) * WAVE_HEIGHT

      // Horizontal ripple
      const horizontalOffset = Math.cos(y / 80 + time * 1.5 * speedMod) * 15

      // Calculate cursor influence - create a "fabric sink" effect
      const dx = x - cursor.x
      const dy = y - cursor.y
      const dist = Math.sqrt(dx * dx + dy * dy)

      let cursorPushX = 0
      let cursorPushY = 0

      if (dist < CURSOR_INFLUENCE) {
        if (dist < CURSOR_PULL_RADIUS) {
          // Pull particles toward cursor in the center (creating the "sink")
          const pullForce = (1 - dist / CURSOR_PULL_RADIUS) * 0.5
          cursorPushX = dist > 0 ? -(dx / dist) * pullForce * 20 : 0
          cursorPushY = dist > 0 ? -(dy / dist) * pullForce * 20 : 0
        }
        else {
          // Push particles away in the outer ring
          const pushForce = 1 - (dist - CURSOR_PULL_RADIUS) / (CURSOR_INFLUENCE - CURSOR_PULL_RADIUS)
          cursorPushX = dist > 0 ? (dx / dist) * pushForce * 40 : 0
          cursorPushY = dist > 0 ? (dy / dist) * pushForce * 40 : 0
        }
      }

      sprite.x = x + horizontalOffset + noiseVal * 10 + cursorPushX
      sprite.y = y + verticalOffset + noiseVal * 15 + cursorPushY

      // Pulsing opacity and scale
      const pulse = (Math.sin(time * 3 + phase) + 1) / 2
      const cursorForce = Math.max(0, 1 - dist / CURSOR_INFLUENCE)
      sprite.alpha = 0.4 + pulse * 0.4 + cursorForce * 0.3
      sprite.scale.set(0.25 + pulse * 0.2)

      // Rotation based on noise
      sprite.rotation = noiseVal * Math.PI
    }
  })

  mountedScope.run(() => {
    useEventListener('mousemove', (e: MouseEvent) => {
      cursor.x = e.clientX
      cursor.y = e.clientY
    })

    useEventListener('resize', () => {
      w = window.innerWidth
      h = window.innerHeight
      addPoints({ textures, container })
    })
    onScopeDispose(() => {
      try {
        app?.destroy(true, { children: true, texture: true, textureSource: true })
      }
      catch (error) {
        console.error(error)
      }
    })
  })
}

onMounted(async () => {
  await setup()
})

onUnmounted(() => {
  mountedScope.stop()
})
</script>

<template>
  <div ref="el" z--1 fixed size-screen left-0 right-0 top-0 bottom-0 pointer-events-none dark:invert />
</template>
