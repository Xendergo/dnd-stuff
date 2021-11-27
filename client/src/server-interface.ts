import { Store } from "./better-store"
import { CAMPAIGNS, CAMPAIGN_NAME, IP_ADDRESS, SERVER_ID } from "./data"

let socket: WebSocket | null = null

export function connect() {
    let address = `ws://${IP_ADDRESS.value}${
        IP_ADDRESS.value.includes(":") ? "" : ":8000"
    }`

    socket = new WebSocket(address)

    socket.onopen = () => {
        if (SERVER_ID.value === null) {
            socket.send(`{"Id": null}`)
        } else {
            socket.send(`{"Id": ${SERVER_ID.value}`)
        }
    }
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
