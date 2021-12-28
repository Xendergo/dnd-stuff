<script lang="ts">
    import { Spell } from "../../spell"

    import type { Store } from "../../better-store"
    import type { Character } from "../../characters"

    export let character: Store<Character>
    export let writeable: boolean
</script>

{#each $character.spells as spell, i}
    <input placeholder="name" bind:value={spell.name} disabled={!writeable} />
    <div class="select-wrapper" disabled={!writeable}>
        <select bind:value={spell.level} disabled={!writeable}>
            <option value="0">Cantrip</option>
            <option value="1">Lvl 1</option>
            <option value="2">Lvl 2</option>
            <option value="3">Lvl 3</option>
            <option value="4">Lvl 4</option>
            <option value="5">Lvl 5</option>
            <option value="6">Lvl 6</option>
            <option value="7">Lvl 7</option>
            <option value="8">Lvl 8</option>
            <option value="9">Lvl 9</option>
        </select>
    </div>

    {#if writeable}
        <button
            on:click={() => {
                character.update(c => {
                    c.spells.splice(i, 1)
                    return c
                })
            }}>🗑️</button
        >
    {/if}
    <br />
{/each}

{#if writeable}
    <button
        on:click={() => {
            character.update(c => {
                c.spells.push(new Spell({}))
                return c
            })
        }}>New spell</button
    >
{/if}
