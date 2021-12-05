export class Character {
    constructor(data, owner = null) {
        this.owner = owner

        this.name = data.name
        this.hp = data.hp ?? null
        this.hp_max = data.hp_max ?? null
        this.initiative = data.initiative ?? null
    }

    owner: number | null

    name: string
    hp_max: number | null
    hp: number | null
    initiative: number | null
}
