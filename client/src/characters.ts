import { Store } from "./better-store"

export class Character {
    constructor(data) {
        this.name = data.name
        this.hp = data.hp
        this.hp_max = data.hp_max
        this.initiative = data.initiative
    }

    name: string
    hp_max?: number
    hp?: number
    initiative?: number
}

export let characters: Store<Character[]> = new Store(
    JSON.parse(localStorage.getItem("characters") ?? "[]").map(
        v => new Character(v)
    ),
    v => localStorage.setItem("characters", JSON.stringify(v))
)
