<script lang="ts">
    import type { Store } from "../../better-store"
    import type { Character } from "../../characters"

    export let character: Store<Character>
    export let writeable: boolean
</script>

{#each Array.from({ length: 9 }, (_, i) => i) as index}
    <div>
        <p>Lvl {index + 1}</p>
        <input type="number" bind:value={$character.spellSlots[index]} />
        {#each Array.from({ length: $character.currentSpellSlots[index] }, (_, i) => i) as slotNum}
            <span>{index + 1}</span>
        {/each}
    </div>
{/each}
<button
    on:click={() => {
        for (let i = 0; i < 9; i++) {
            $character.currentSpellSlots[i] = $character.spellSlots[i]
        }
    }}>🔄</button
>

<style>
    p {
        margin: 8px;
    }

    div {
        display: flex;
        align-items: center;
    }

    span {
        border-radius: 50%;
        width: calc(1.5rem + 20px);
        height: calc(1.5rem + 20px);
        display: inline-block;

        border: 2px solid white;

        margin-left: 8px;

        display: inline-flex;
        align-items: center;
        justify-content: center;

        cursor: grab;
    }
</style>
