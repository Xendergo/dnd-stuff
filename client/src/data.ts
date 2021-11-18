import { Store } from "./better-store"
import { Character } from "./characters"

export let IS_GM = new Store(
    sessionStorage.getItem("is_gm") == "true" ? true : false,
    data => sessionStorage.setItem("is_gm", data ? "true" : "false")
)

export let GAME_NAME: Store<string | null> = new Store(
    sessionStorage.getItem("game_name"),
    data => {
        if (data) {
            sessionStorage.setItem("game_name", data)
        } else {
            sessionStorage.removeItem("game_name")
        }
    }
)

let games_gming_raw = JSON.parse(
    localStorage.getItem("games_gming") ?? "{}"
) as { [key: string]: string[] }

let games_gming = {} as { [key: string]: Character[] }

Object.keys(games_gming_raw).forEach(key => {
    games_gming[key] = games_gming_raw[key].map(v => new Character(v))
})

export let GAMES_GMING = new Store(games_gming, data =>
    localStorage.setItem("games_gming", JSON.stringify(data))
)
