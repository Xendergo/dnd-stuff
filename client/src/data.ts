import { Store } from "./better-store"
import { Character } from "./characters"

export const urlParams = new URLSearchParams(location.search)

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
