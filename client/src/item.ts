export class Item {
    constructor(data) {
        this.name = data.name ?? ""
        this.unitWeight = data.unitWeight ?? 0
        this.quantity = data.quantity ?? 1
    }

    name: string
    unitWeight: number
    quantity: number
}
