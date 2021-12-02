import { Store } from "./better-store"
import { CAMPAIGNS, CAMPAIGN_NAME, IP_ADDRESS, SERVER_ID } from "./data"
import {
    AbstractListenerManager,
    MakeSendable,
    Registry,
    Sendable,
    strats,
} from "triangulum"
import type { Character } from "./characters"

export let socket: ConnectionManager | null = null

const messageRegistry = new Registry<Sendable>()

class ConnectionManager extends AbstractListenerManager<
    Sendable,
    object,
    string
> {
    constructor() {
        super(messageRegistry)

        let address = `ws://${IP_ADDRESS.value}${
            IP_ADDRESS.value.includes(":") ? "" : ":8000"
        }`

        this.ws = new WebSocket(address)

        this.ws.onmessage = e => {
            if (typeof e.data !== "string") return

            console.log(e.data)

            this.onData(e.data as string)
        }

        this.ws.onopen = e => {
            this.ready()
        }
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
        if (!typeCheckingLayers[0](data)) {
            throw new Error("Type checking failed")
        }

        return data as Sendable
    }

    ws
}

@MakeSendable(
    messageRegistry,
    "Id",
    strats.class({
        id: strats.isNumber,
    })
)
export class Id extends Sendable {
    constructor(id: number) {
        super()
        this.id = id
    }

    id: number
}

@MakeSendable(messageRegistry, "RequestId", strats.dontCheck())
export class RequestId extends Sendable {
    constructor() {
        super()
    }
}

@MakeSendable(
    messageRegistry,
    "CharacterUpdated",
    strats.class({
        data: strats.isString,
    })
)
export class CharacterUpdated extends Sendable {
    constructor(data: Character) {
        super()
        this.data = JSON.stringify(data)
    }

    data: string
}

export function connect() {
    socket = new ConnectionManager()

    if (SERVER_ID.value === null) {
        socket.send(new RequestId())
    } else {
        socket.send(new Id(SERVER_ID.value))
    }

    socket.listen(Id, id => SERVER_ID.set(id.id))
}

console.log("Trying to connect")
if (IP_ADDRESS.value) {
    connect()
}

export function getCharacters() {
    if (socket === null) {
        let characters = CAMPAIGNS.value[CAMPAIGN_NAME]

        return characters.map(
            v => new Store(v, () => CAMPAIGNS.notifySubscribers())
        )
    } else {
        let localCharacters = CAMPAIGNS.value[CAMPAIGN_NAME]

        let localCharactersStores = localCharacters.map(
            v => new Store(v, () => CAMPAIGNS.notifySubscribers())
        )

        localCharactersStores.forEach(store => {
            store.subscribe(v => {
                socket.send(new CharacterUpdated(v))
            })
        })

        // TODO: Include stores from other connected users

        return localCharactersStores
    }
}
