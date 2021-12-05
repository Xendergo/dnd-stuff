import { ProceduralStore, Store } from "./better-store"
import {
    CAMPAIGNS,
    CAMPAIGN_NAME,
    IP_ADDRESS,
    SERVER_ID as CLIENT_ID,
} from "./data"
import { Character } from "./characters"
import { ConnectionManager } from "./socket"
import { CharacterUpdated, Id, RequestId } from "./sendable-types"

export let socket: ConnectionManager | null = null

export function connect() {
    console.log("Trying to connect")

    try {
        socket = new ConnectionManager()
    } catch {
        IP_ADDRESS.set(null)
        return
    }

    if (CLIENT_ID.value === null) {
        socket.send(new RequestId())
    } else {
        socket.send(new Id(CLIENT_ID.value))
    }

    socket.listen(Id, id => CLIENT_ID.set(id.id))

    socket.listen(CharacterUpdated, onCharacterUpdated)

    characterList.nextValueAvailable()

    if (CAMPAIGN_NAME !== null) {
        CAMPAIGNS.value[CAMPAIGN_NAME].forEach(character => {
            socket!.send(new CharacterUpdated(character))
        })
    }
}

let networkCharacters: Character[] = []

class CharacterList extends ProceduralStore<Store<Character>[] | null> {
    protected next(): Store<Character>[] | null {
        if (CAMPAIGN_NAME === null) {
            return null
        }

        if (socket === null) {
            let characters = CAMPAIGNS.value[CAMPAIGN_NAME]

            return characters.map(
                v => new Store(v, () => CAMPAIGNS.notifySubscribers())
            )
        } else {
            let localCharacters = CAMPAIGNS.value[CAMPAIGN_NAME]

            let localCharactersStores = localCharacters.map(
                v =>
                    new Store(v, updatedCharacter => {
                        CAMPAIGNS.notifySubscribers()
                        socket!.send(new CharacterUpdated(updatedCharacter))
                    })
            )

            let networkCharactersStores = networkCharacters.map(
                v => new Store(v)
            )

            return [...localCharactersStores, ...networkCharactersStores]
        }
    }
}

export let characterList: CharacterList = new CharacterList()

function onCharacterUpdated(characterUpdated: CharacterUpdated) {
    if (characterUpdated.player_id === CLIENT_ID.value) {
        return
    }

    let decoded = new Character(JSON.parse(characterUpdated.data))

    let indexOfCharacter = networkCharacters.findIndex(
        v => v.name === decoded.name
    )

    if (indexOfCharacter === -1) {
        networkCharacters.push(decoded)
    } else {
        networkCharacters[indexOfCharacter] = decoded

        networkCharacters = networkCharacters.filter(
            (v, i) => i <= indexOfCharacter || v.name !== decoded.name
        )
    }

    characterList.nextValueAvailable()
}

if (IP_ADDRESS.value) {
    connect()
}
