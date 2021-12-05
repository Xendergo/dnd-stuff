import { Store } from "./better-store"
import { Character } from "./characters"

const urlParams = new URLSearchParams(location.search)

let campaigns_raw = JSON.parse(
    localStorage.getItem("campaigns_gming") ?? "{}"
) as { [key: string]: string[] }

let campaigns = {} as { [key: string]: Character[] }

Object.keys(campaigns_raw).forEach(key => {
    campaigns[key] = campaigns_raw[key].map(v => new Character(v))
})

export let CAMPAIGNS = new Store(campaigns, data =>
    localStorage.setItem("campaigns_gming", JSON.stringify(data))
)

let ip = sessionStorage.getItem("ip")

export let IP_ADDRESS = new Store(ip === "" ? null : ip, data => {
    if (data === null) {
        sessionStorage.removeItem("ip")
    } else {
        sessionStorage.setItem("ip", data)
    }
})

export let IS_GM = new Store(sessionStorage.getItem("gm") === "true", v =>
    sessionStorage.setItem("gm", v === true ? "true" : "false")
)

let maybe_id = parseInt(sessionStorage.getItem("server_id")!)

export let SERVER_ID = new Store(isNaN(maybe_id) ? null : maybe_id, v => {
    if (v === null) {
        sessionStorage.removeItem("server_id")
    } else {
        sessionStorage.setItem("server_id", v.toString())
    }
})

export let CAMPAIGN_NAME = urlParams.get("campaign") ?? null

export let CHARACTER_NAME = urlParams.get("character") ?? null
