<script lang="ts">
    import type { Store } from "src/better-store"
    import type { Character } from "src/characters"

    export let character: Store<Character>
    export let writeable: boolean
    export let index: number

    function dragStart(e) {
        e.dataTransfer.setData("text/plain", index)

        requestAnimationFrame(() => {
            e.target.style.visibility = "hidden"
        })
    }

    function dragEnd(e) {
        requestAnimationFrame(() => {
            e.target.style.visibility = "visible"
        })
    }

    function dropOnRow(e) {
        $character.currentSpellSlots[index]++

        const droppedIndex = e.dataTransfer.getData("text/plain")

        if (droppedIndex !== null) {
            $character.currentSpellSlots[parseInt(droppedIndex)]--
        }
    }
</script>

<div
    class="level-row"
    on:dragover|preventDefault
    on:drop|preventDefault={dropOnRow}
>
    <p>Lvl {index + 1}</p>
    <input
        type="number"
        bind:value={$character.spellSlots[index]}
        disabled={!writeable}
    />
    {#each Array.from({ length: $character.currentSpellSlots[index] }, (_, i) => i) as slotNum}
        <span
            class="circle"
            draggable={writeable}
            on:dragstart={dragStart}
            on:dragend={dragEnd}>{index + 1}</span
        >
    {/each}
</div>

<style>
    p {
        margin: 8px;
    }

    .level-row {
        display: flex;
        align-items: center;
    }
</style>
