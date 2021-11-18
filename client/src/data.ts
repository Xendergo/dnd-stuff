import { Store } from "./better-store"
import { Character } from "./characters"

export let IS_GM = new Store(
    sessionStorage.getItem("is_gm") == "true" ? true : false,
    data => sessionStorage.setItem("is_gm", data ? "true" : "false")
)

export let CAMPAIGN_NAME: Store<string | null> = new Store(
    sessionStorage.getItem("game_name"),
    data => {
        if (data) {
            sessionStorage.setItem("game_name", data)
        } else {
            sessionStorage.removeItem("game_name")
        }
    }
)

let campaigns_gming_raw = JSON.parse(
    localStorage.getItem("campaigns_gming") ?? "{}"
) as { [key: string]: string[] }

let campaigns_gming = {} as { [key: string]: Character[] }

Object.keys(campaigns_gming_raw).forEach(key => {
    campaigns_gming[key] = campaigns_gming_raw[key].map(v => new Character(v))
})

export let CAMPAIGNS_GMING = new Store(campaigns_gming, data =>
    localStorage.setItem("campaigns_gming", JSON.stringify(data))
)
