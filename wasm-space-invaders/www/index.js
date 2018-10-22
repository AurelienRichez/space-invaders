import { App } from 'wasm-space-invaders'

const app = App.new()
const canvas = document.getElementById("space-invaders-canvas")
const ctx = canvas.getContext('2d')

const renderLoop = () => {
  app.draw(ctx)
  requestAnimationFrame(renderLoop)
  ctx.stroke();
}

let lastRun = performance.now()
const tick = () => {
  const currentTime = performance.now()
  app.run(currentTime - lastRun)
  lastRun = currentTime
}

requestAnimationFrame(renderLoop)
setInterval(tick, 8)