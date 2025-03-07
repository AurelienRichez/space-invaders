import init, { App } from "./pkg/wasm_space_invaders.js";
init().then(() => {
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

  const handleKeyDown = (e) => {
    if(!e.repeat) {
      app.handle_key_down(e.code)
    }
  }

  const handleKeyUp = (e) => {
    app.handle_key_up(e.code)
  }

  requestAnimationFrame(renderLoop)
  setInterval(tick, 1)
  document.addEventListener('keydown', handleKeyDown)
  document.addEventListener('keyup', handleKeyUp)
});
