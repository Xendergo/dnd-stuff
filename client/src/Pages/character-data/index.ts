import App from "./CharacterData.svelte"
import "../../dice"

const app = new App({
    target: document.getElementById("app")!,
})

let displacement: number | null = null
let displacementTimeout: number | undefined = undefined

window.addEventListener("wheel", e => {
    e.preventDefault()

    if (displacement === null) {
        displacement = document.documentElement.scrollLeft + e.deltaY + e.deltaX
    } else {
        displacement = displacement + e.deltaY + e.deltaX
    }

    clearTimeout(displacementTimeout)
    displacementTimeout = setTimeout(() => {
        displacement = null
    }, 500) as unknown as number
})

setInterval(() => {
    if (displacement !== null) {
        scrollBy((displacement - document.documentElement.scrollLeft) * 0.15, 0)
    }
}, 1000 / 60)

export default app
