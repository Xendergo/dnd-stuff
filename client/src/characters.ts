import { Store } from "./better-store"

export let characters: Store<Character[]> = new Store(
    JSON.parse(localStorage.getItem("characters") ?? "[]").map(
        v => new Character(v)
    )
)

export class Character {
    constructor(data) {
        this.name = data.name
    }

    name: string
}
