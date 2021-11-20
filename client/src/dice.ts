type diceSyntaxToken =
    | ["const", number]
    | ["roll", number, number]
    | "+"
    | "-"
    | "*"

type groupedDiceSyntaxToken = diceSyntaxToken | groupedDiceSyntax

type diceSyntax = diceSyntaxToken[]

type groupedDiceSyntax = groupedDiceSyntaxToken[]

export function roll(dice: string): number | null {
    let fn = parseDiceSyntax(dice)

    if (fn === null) {
        return null
    }

    return fn()
}

const cache = new Map<string, () => number>()

export function parseDiceSyntax(dice: string): (() => number) | null {
    let formatted = dice.toLowerCase().replaceAll(" ", "")

    if (cache.has(formatted)) {
        return cache.get(formatted)
    }

    let syntax = tokenizeDiceSyntax(formatted)

    let fn = syntaxTokensToFunction(syntax)

    cache.set(formatted, fn)

    return fn
}

function tokenizeDiceSyntax(dice: string): diceSyntax | null {
    let syntax: diceSyntax = []

    // Parse the dice syntax
    for (let i = 0; i < dice.length; ) {
        if (dice[i] === "d") {
            // Roll a dice: `d20`, `d12`

            let data = parseDice(dice.slice(i))

            if (data === null) {
                return null
            }

            syntax.push(data[0])

            i += data[1]
        } else if (dice[i].match(/\d/g) !== null) {
            // Start of a number or multiple dice rolls: `4`, `3d4`

            let data = parseNumber(dice.slice(i))

            if (data === null) {
                return null
            }

            syntax.push(data[0])

            i += data[1]
        } else if (dice[i].match(/[\+\-\*]/g) !== null) {
            // Operator

            syntax.push(dice[i] as "+" | "-" | "*")

            i++
        } else {
            // bad syntax
            return null
        }
    }

    return syntax
}

function syntaxTokensToFunction(
    tokens: groupedDiceSyntax
): (() => number) | null {
    // Step 1: Group the tokens into a tree structure
    let parsedTokens = groupTokens(tokens)

    if (parsedTokens === false) {
        return null
    }

    return walkSyntaxTree(parsedTokens)
}

function groupTokens(token: groupedDiceSyntaxToken): groupedDiceSyntax | false {
    if (!Array.isArray(token) || token[0] === "const" || token[0] === "roll") {
        return [token]
    }

    let operator = token.indexOf("*")

    if (operator === -1) {
        operator = token.findIndex(v => v === "+" || v === "-")
    }

    if (operator !== -1) {
        if (operator < 1 || operator > token.length - 2) {
            return false
        }

        let op1 = groupTokens(token[operator - 1])
        let op = token[operator]
        let op2 = groupTokens(token[operator + 1])

        if (op1 === false || op2 === false) {
            return false
        }

        token.splice(operator - 1, 3, [op1, op, op2])

        if (!Array.isArray(op1) || !Array.isArray(op2)) {
            return false
        }

        return groupTokens(token)
    }

    return token
}

function walkSyntaxTree(tree: groupedDiceSyntax): () => number | null {
    if (tree.length === 1 && Array.isArray(tree[0])) {
        if (tree[0][0] === "const") {
            return constFn(tree[0][1])
        } else if (tree[0][0] === "roll") {
            return rollFn(tree[0][1], tree[0][2])
        }

        return walkSyntaxTree(tree[0])
    } else if (
        tree.length === 3 &&
        Array.isArray(tree[0]) &&
        typeof tree[1] === "string" &&
        Array.isArray(tree[2])
    ) {
        let fn1 = walkSyntaxTree(tree[0] as groupedDiceSyntax)
        let fn2 = walkSyntaxTree(tree[2] as groupedDiceSyntax)

        if (tree[1] === "+") {
            return addFn(fn1, fn2)
        } else if (tree[1] === "-") {
            return subFn(fn1, fn2)
        } else if (tree[1] === "*") {
            return multFn(fn1, fn2)
        }
    }
}

function constFn(constant: number): () => number {
    return () => constant
}

function rollFn(sides: number, times: number): () => number {
    return () => {
        let sum = 0

        for (let i = 0; i < times; i++) {
            sum += Math.floor(Math.random() * sides + 1)
        }

        return sum
    }
}

function addFn(fn1: () => number, fn2: () => number): () => number {
    return () => fn1() + fn2()
}

function subFn(fn1: () => number, fn2: () => number): () => number {
    return () => fn1() - fn2()
}

function multFn(fn1: () => number, fn2: () => number): () => number {
    return () => fn1() * fn2()
}

function parseDice(slice: string): [diceSyntaxToken, number] | null {
    let i = 1

    let dice_sides = slice.slice(i).match(/\d+/g)

    if (dice_sides === null) {
        return null
    }

    i += dice_sides[0].length

    return [["roll", parseInt(dice_sides[0]), 1], i]
}

function parseNumber(slice: string): [diceSyntaxToken, number] | null {
    let i = 0

    let num_str = slice.slice(i).match(/\d+/g)[0]
    let num = parseInt(num_str)

    i += num_str.length

    if (slice[i] === "d") {
        // This is an amount of dice rolls: `3d4`

        let data = parseDice(slice.slice(i))

        if (data === null) {
            return null
        }

        i += data[1]

        return [["roll", data[0][1] as number, num], i]
    } else {
        // This number is a constant to add or multiply or whatever

        return [["const", num], i]
    }
}
