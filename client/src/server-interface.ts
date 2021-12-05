import { ProceduralStore, Store } from "./better-store"
import { CAMPAIGNS, CAMPAIGN_NAME, IP_ADDRESS, CLIENT_ID } from "./data"
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

    CLIENT_ID.subscribe(id => {
        if (CAMPAIGN_NAME !== null && id !== null) {
            CAMPAIGNS.value[CAMPAIGN_NAME].forEach(character => {
                character.owner = id
                socket!.send(new CharacterUpdated(character))
            })
        }
    })

    if (CLIENT_ID.value === null) {
        socket.send(new RequestId())
    } else {
        socket.send(new Id(CLIENT_ID.value))
    }

    socket.listen(Id, id => {
        CLIENT_ID.set(id.id)
    })

    socket.listen(CharacterUpdated, onCharacterUpdated)

    characterList.nextValueAvailable()
}

/*
 * `networkCharacters` gets cleared when someone clicks a link to a character's data,
 *  so the data has to be cached so it can be retrieved again without waiting for the server
 */

let networkCharacterCache = JSON.parse(
    sessionStorage.getItem("networkCharacterCache") ?? "[]"
).map(
    v =>
        new Store(new Character(v), () => networkCharacters.notifySubscribers())
)

let networkCharacters: Store<Store<Character>[]> = new Store(
    networkCharacterCache,
    v => {
        sessionStorage.setItem(
            "networkCharacterCache",
            JSON.stringify(v.map(character => character.value))
        )
    }
)

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

            localCharacters.forEach(character => {
                character.owner = CLIENT_ID.value
            })

            let localCharactersStores = localCharacters.map(
                v =>
                    new Store(v, updatedCharacter => {
                        CAMPAIGNS.notifySubscribers()
                        socket!.send(new CharacterUpdated(updatedCharacter))
                    })
            )

            let networkCharactersStores = networkCharacters.value

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

    let indexOfCharacter = networkCharacters.value.findIndex(
        v => v.value.name === decoded.name
    )

    if (indexOfCharacter === -1) {
        networkCharacters.update(v => {
            v.push(new Store(decoded, networkCharacters.notifySubscribers))
            return v
        })
    } else {
        networkCharacters.value[indexOfCharacter].set(decoded)

        networkCharacters.update(characters =>
            characters.filter(
                (characterTesting, i) =>
                    i <= indexOfCharacter ||
                    characterTesting.value.name !== decoded.name
            )
        )
    }

    characterList.nextValueAvailable()
}

if (IP_ADDRESS.value) {
    connect()
}
