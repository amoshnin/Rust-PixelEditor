const cellSize = 50

const colors = {
  red: [255, 200, 200],
  green: [200, 255, 200],
  purple: [200, 200, 255],
}

function draw(state) {
  const canvas = document.getElementById('main-canvas')
  const context = canvas.getContext('2d')

  context.strokeStyle = 'black'
  context.lineWidth = 1

  const image = state.image
  const width = image.width()
  const height = image.height()

  const cells = image.cells()

  for (let x = 0; x < width; x++) {
    for (let y = 0; y < height; y++) {
      const index = (y * width + x) * 3
      const r = cells[index + 0]
      const g = cells[index + 1]
      const b = cells[index + 2]
      const color = `rgb(${r}, ${g}, ${b})`
      context.fillStyle = color
      context.fillRect(x * cellSize, y * cellSize, cellSize, cellSize)
    }
  }

  for (let x = 0; x <= width; x++) {
    context.beginPath()
    context.moveTo(x * cellSize + 0.5, 0)
    context.lineTo(x * cellSize + 0.5, height * cellSize)
    context.stroke()
  }

  for (let y = 0; y <= height; y++) {
    context.beginPath()
    context.moveTo(0, y * cellSize + 0.5)
    context.lineTo(width * cellSize, y * cellSize + 0.5)
    context.stroke()
  }
}

function setupCanvas(state) {
  const canvas = document.getElementById('main-canvas')

  function coloring() {
    const rect = canvas.getBoundingClientRect()

    let x = event.clientX - rect.left
    let y = event.clientY - rect.top

    x = Math.floor(x / cellSize)
    y = Math.floor(y / cellSize)

    const image = state.image
    image.brush(x, y, state.currentColor)
    draw(state)
  }

  canvas.addEventListener('click', coloring)

  canvas.addEventListener('mousedown', () => (state.dragging = true))

  canvas.addEventListener('mouseup', () => (state.dragging = false))

  canvas.addEventListener('mousemove', () => {
    if (!state.dragging) return
    coloring()
  })

  document
    .getElementById('red')
    .addEventListener('click', () => (state.currentColor = colors.red))
  document
    .getElementById('green')
    .addEventListener('click', () => (state.currentColor = colors.green))
  document
    .getElementById('purple')
    .addEventListener('click', () => (state.currentColor = colors.purple))
}

async function main() {
  const lib = await import('../pkg/index.js').catch(console.error)
  const image = new lib.Image(10, 10)
  const state = {
    image,
    currentColor: [200, 255, 200],
    dragging: false,
  }
  setupCanvas(state)
  draw(state)
}

main()
