import type { Readable, Writable } from "svelte/store"

class BaseStore<T> implements Readable<T> {
    protected subscribers: ((value: T) => void)[] = []

    protected _value: T

    notifySubscribers() {
        this.subscribers.forEach(v => {
            v(this._value)
        })
    }

    subscribe(run: (value: T) => void) {
        run(this._value)

        return this.onChange(run)
    }

    onChange(run: (value: T) => void) {
        this.subscribers.push(run)

        return () => {
            let index = this.subscribers.indexOf(run)

            if (index != -1) {
                this.subscribers.splice(index, 1)
            }
        }
    }
}

export class Store<T> extends BaseStore<T> implements Writable<T> {
    constructor(value: T, write?: (value: T) => void) {
        super()

        this._value = value

        if (write) {
            this.subscribers.push(write)
        }
    }

    get value() {
        return this._value
    }

    set value(value: T) {
        this._value = value
        this.notifySubscribers()
    }

    set(v: T) {
        this.value = v
    }

    update(updater: (value: T) => T) {
        this.value = updater(this.value)
    }
}

export abstract class ProceduralStore<T> extends BaseStore<T> {
    constructor() {
        super()
        this.nextValueAvailable()
    }

    get value() {
        return this._value
    }

    protected abstract next(): T

    nextValueAvailable() {
        this._value = this.next()
        this.notifySubscribers()
    }
}
