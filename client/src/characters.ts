export class Character {
    constructor(data, readonly = false) {
        this.readonly = readonly

        this.name = data.name
        this.hp = data.hp ?? null
        this.hp_max = data.hp_max ?? null
        this.initiative = data.initiative ?? null
    }

    readonly: boolean

    name: string
    hp_max: number | null
    hp: number | null
    initiative: number | null
}
