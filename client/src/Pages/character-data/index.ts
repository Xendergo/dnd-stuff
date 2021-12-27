import App from "./CharacterData.svelte"
import "../../dice"

const app = new App({
    target: document.getElementById("app")!,
})

window.addEventListener("wheel", e => {
    e.preventDefault()

    scroll({
        top: 0,
        left: document.documentElement.scrollLeft - e.deltaY + e.deltaX,
        behavior: "smooth",
    })
})

export default app
