<script lang="ts">
    import type { Store } from "../../../better-store"
    import type { Character } from "../../../characters"
    import CastSpellButton from "./CastSpellButton.svelte"
    import LevelRow from "./LevelRow.svelte"

    export let character: Store<Character>
    export let writeable: boolean
</script>

{#each Array.from({ length: 9 }, (_, i) => i) as index}
    <LevelRow bind:character {writeable} {index} />
{/each}

{#if writeable}
    <button
        on:click={() => {
            for (let i = 0; i < 9; i++) {
                $character.currentSpellSlots[i] = $character.spellSlots[i]
            }
        }}>🔄</button
    >

    <div id="drop-row">
        <span class="circle" draggable="true" />
        <CastSpellButton bind:character />
    </div>
{/if}

<style>
    #drop-row {
        position: absolute;
        bottom: 16px;
        right: 16px;

        display: flex;
        justify-content: center;
    }
</style>
