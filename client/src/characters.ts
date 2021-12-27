import { Item } from "./item"
import { Spell } from "./spell"

export class Character {
    constructor(data) {
        this.owner = data.owner ?? null
        this.name = data.name

        this.hp = data.hp ?? null
        this.hp_max = data.hp_max ?? null
        this.initiative = data.initiative ?? null

        this.items = (data.items ?? []).map(v => new Item(v))

        this.spells = (data.spells ?? []).map(v => new Spell(v))
    }

    owner: number | null
    name: string

    hp_max: number | null
    hp: number | null
    initiative: number | null

    items: Item[]

    spells: Spell[]
}
