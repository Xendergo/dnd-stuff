import { Store } from "./better-store"
import {
    CAMPAIGNS,
    CAMPAIGN_NAME,
    CHARACTER_NAME,
    IP_ADDRESS,
    IS_GM,
} from "./data"

let socket: WebSocket | null = null

export function connect() {
    let address = `ws://${IP_ADDRESS.value}${
        IP_ADDRESS.value.includes(":") ? "" : ":8000"
    }`

    socket = new WebSocket(address)
}

if (IP_ADDRESS.value) {
    connect()
}

export function getCharacters() {
    if (socket === null) {
        let characters = CAMPAIGNS.value[CAMPAIGN_NAME]

        return characters.map(
            v => new Store(v, () => CAMPAIGNS.notifySubscribers())
        )
    }
}
