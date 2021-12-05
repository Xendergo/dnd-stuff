import { Sendable, strats } from "triangulum"
import type { Character } from "./characters"
import { Message } from "./socket"

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
