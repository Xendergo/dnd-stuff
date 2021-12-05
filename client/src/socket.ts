import {
    AbstractListenerManager,
    makeCustomSendableDecorator,
    Registry,
    Sendable,
} from "triangulum"
import { IP_ADDRESS, SERVER_ID } from "./data"

const messageRegistry = new Registry<Sendable>()

export const Message = makeCustomSendableDecorator(messageRegistry)

export class ConnectionManager extends AbstractListenerManager<
    Sendable,
    object,
    string
> {
    constructor() {
        super(messageRegistry)

        let address = `ws://${IP_ADDRESS.value?.trim()}${
            IP_ADDRESS.value!.includes(":") ? "" : ":8000"
        }`

        this.ws = new WebSocket(address)

        this.ws.onmessage = e => {
            if (typeof e.data !== "string") return

            this.onData(e.data)
        }

        this.ws.onopen = e => {
            this.ready()
        }

        this.ws.onerror = this.disconnect
    }

    disconnect() {
        SERVER_ID.set(null)
        IP_ADDRESS.set(null)
    }

    transmit(data: string) {
        this.ws.send(data)
    }

    encode(data: Sendable): string {
        return `{"${data.channel}": ${JSON.stringify(data)}}`
    }

    decode(data: string): [string, object] {
        let data_parsed = JSON.parse(data)

        let keys = Object.keys(data_parsed)

        if (keys.length === 0) {
            throw new Error("No data")
        }

        return [keys[0], data_parsed[keys[0]]]
    }

    finalize(
        data: object,
        typeCheckingLayers: (data: any) => boolean
    ): Sendable {
        if (!typeCheckingLayers(data)) {
            throw new Error("Type checking failed")
        }

        return data as Sendable
    }

    ws: WebSocket
}
