<script lang="ts">
    import { Item } from "../../item"
    import type { Store } from "../../better-store"
    import type { Character } from "../../characters"

    export let character: Store<Character>
    export let writeable: boolean
</script>

{#each $character.items as item, i}
    <input type="number" bind:value={item.quantity} disabled={!writeable} />
    <input placeholder="name" bind:value={item.name} disabled={!writeable} />
    {item.unitWeight * item.quantity} lbs,
    <input type="number" bind:value={item.unitWeight} disabled={!writeable} />

    {#if writeable}
        <button
            on:click={() => {
                character.update(c => {
                    c.items.splice(i, 1)
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
                c.items.push(new Item({}))
                return c
            })
        }}>New item</button
    >
{/if}
