import type { Writable } from "svelte/store"

export class Store<T> implements Writable<T> {
    constructor(value: T, write?: (value: T) => void) {
        this._value = value

        if (write) {
            this.subscribe(write)
        }
    }

    private _value: T
    private subscribers: ((value: T) => void)[] = []

    get value() {
        return this._value
    }

    set value(value) {
        this._value = value
        this.notifySubscribers()
    }

    notifySubscribers() {
        this.subscribers.forEach(v => {
            v(this.value)
        })
    }

    subscribe(run: (value: T) => void) {
        this.subscribers.push(run)

        run(this.value)

        return () => {
            let index = this.subscribers.indexOf(run)

            if (index != -1) {
                this.subscribers.splice(index, 1)
            }
        }
    }

    set(v) {
        this.value = v
    }

    update(updater: (value: T) => T) {
        this.value = updater(this.value)
    }
}
