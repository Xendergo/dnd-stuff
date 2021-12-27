export class Spell {
    constructor(data) {
        this.name = data.name ?? ""
        this.level = data.level ?? "0"
    }

    name: string
    level: "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
}
