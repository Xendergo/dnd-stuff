import { Store } from "./better-store"

export class Character {
    constructor(data) {
        this.name = data.name
    }

    name: string
}

export let characters: Store<Character[]> = new Store(
    JSON.parse(localStorage.getItem("characters") ?? "[]").map(
        v => new Character(v)
    ),
    v => localStorage.setItem("characters", JSON.stringify(v))
)
