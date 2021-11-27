import { Store } from "./better-store"

export class Character {
    constructor(data) {
        this.name = data.name
        this.hp = data.hp ?? null
        this.hp_max = data.hp_max ?? null
        this.initiative = data.initiative ?? null
    }

    name: string
    hp_max: number | null
    hp: number | null
    initiative: number | null
}
