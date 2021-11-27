import { IP_ADDRESS } from "./data"

export function connect() {
    let address = `ws://${IP_ADDRESS.value}${
        IP_ADDRESS.value.includes(":") ? "" : ":8000"
    }`

    console.log(address)

    let socket = new WebSocket(address)

    socket.onmessage = msg => console.log(msg)

    socket.onopen = () => socket.send("Poggers")
}
