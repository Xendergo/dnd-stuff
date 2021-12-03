import { Store } from "./better-store"
import { CAMPAIGNS, CAMPAIGN_NAME, IP_ADDRESS, SERVER_ID } from "./data"
import { Sendable, strats } from "triangulum"
import { Character } from "./characters"
import { ConnectionManager, Message } from "./socket"

export let socket: ConnectionManager | null = null

@Message(
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

@Message("RequestId", strats.dontCheck())
export class RequestId extends Sendable {
    constructor() {
        super()
    }
}

@Message(
    "CharacterUpdated",
    strats.class({
        data: strats.isString,
        player_id: strats.isNumber,
    })
)
export class CharacterUpdated extends Sendable {
    constructor(data: Character) {
        super()
        this.data = JSON.stringify(data)
    }

    data: string
    player_id: number | undefined = undefined
}

export function connect() {
    socket = new ConnectionManager()

    console.log(SERVER_ID.value)
    if (SERVER_ID.value === null) {
        socket.send(new RequestId())
    } else {
        socket.send(new Id(SERVER_ID.value))
    }

    socket.listen(Id, id => SERVER_ID.set(id.id))

    socket.listen(CharacterUpdated, onCharacterUpdated)
}

let networkCharacters: Character[] = []

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

function onCharacterUpdated(characterUpdated: CharacterUpdated) {
    if (characterUpdated.player_id === SERVER_ID.value) {
        console.log("RECEIVED OWN DATA")
        return
    }

    let decoded = new Character(JSON.parse(characterUpdated.data))

    let indexOfCharacter = networkCharacters.findIndex(
        v => v.name === decoded.name
    )

    if (indexOfCharacter === -1) {
        networkCharacters.push(decoded)
        return
    }

    networkCharacters[indexOfCharacter] = decoded

    networkCharacters = networkCharacters.filter(
        (v, i) => i <= indexOfCharacter || v.name !== decoded.name
    )
}
