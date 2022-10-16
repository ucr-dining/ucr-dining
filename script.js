class Timer {
  constructor() {
    this.timer = document.createElement("div")
    this.timer.classList.add("timer")
    document.body.append(this.timer)
    this.windowFunctionHandle = null
    this.start = null

    const run = (timestamp) => {
      if (this.start === null) {
        this.start = timestamp
      }
      const elapsed = timestamp - this.start
      this.timer.innerText = `Loading... ${elapsed.toFixed(2)}ms and counting`
      this.windowFunctionHandle = window.requestAnimationFrame(run)
    }
    window.requestAnimationFrame(run)
  }
  remove() {
    this.timer.remove()
    window.cancelAnimationFrame(this.windowFunctionHandle)
  }
}

;(async () => {
  const timer = new Timer()
  try {
    // append div to body to signal to the user the amount of time taken
    const res = await fetch(
      "https://bvhcowgtrlriiknk57vtftp5de0nzdpz.lambda-url.us-west-2.on.aws/"
    )

    console.log("request completed successfully")
    timer.remove()

    const { lothian, glasgow } = await res.json()

    ;["breakfast", "lunch", "dinner"].forEach((x) => {
      document.body.append(
        diffDisplay(
          `Lothian ${x}`,
          lothian[x] ?? [],
          `Glasgow ${x}`,
          glasgow[x] ?? []
        )
      )
    })
  } catch (e) {
    window.cancelAnimationFrame(timer.windowFunctionHandle)
    timer.timer.innerText = `${e}`
  }
})()

function createItem(name) {
  let o = document.createElement("div")
  o.classList.add("comparisonItem")
  o.innerText = name
  return o
}

function diffDisplay(title1, array1, title2, array2) {
  // create
  let main = document.createElement("div")
  main.classList.add("comparisonTable")

  let divs = [title1, "shared", title2].map((x) => {
    const div = document.createElement("div")
    div.classList.add("comparisonTableColumn")

    const title = document.createElement("div")
    title.classList.add("comparisonTableColumnTitle")
    title.innerText = x
    div.append(title)
    return div
  })
  let [div1, divShared, div2] = divs

  let set = new Set()
  for (const c of array1) set.add(c)
  for (const c of array2) {
    // console.log(c, "is", set.has(c) ? "shared" : "only y");
    let e = createItem(c)
    e.classList.add("comparisonItem")
    if (set.has(c)) {
      divShared.append(e)
    } else {
      div2.append(e)
    }
    set.delete(c)
  }
  for (const c of set) div1.append(createItem(c))

  main.append(...divs)

  return main
}
