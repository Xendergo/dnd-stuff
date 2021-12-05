<script lang="ts">
    import type { Store } from "src/better-store"

    import type { Character } from "src/characters"
    import { roll } from "../../dice"

    export let character: Store<Character>
    export let writeable: boolean
</script>

<table>
    <tr>
        <td>
            <p>
                HP <input
                    type="number"
                    bind:value={$character.hp}
                    disabled={!writeable}
                />

                {#if writeable}
                    <button on:click={() => ($character.hp -= 1)}>-1</button>
                    <button on:click={() => ($character.hp += 1)}>+1</button>
                    <button on:click={() => ($character.hp = $character.hp_max)}
                        >🔄</button
                    >
                {/if}
            </p>
        </td>
    </tr>
    <tr>
        <td>
            <p>
                max HP <input
                    type="number"
                    bind:value={$character.hp_max}
                    disabled={!writeable}
                />
            </p>
        </td>
    </tr>
    <tr>
        <td>
            <p>
                Initiative <input
                    type="number"
                    min="1"
                    max="20"
                    bind:value={$character.initiative}
                    disabled={!writeable}
                />

                {#if writeable}
                    <button
                        on:click={() => {
                            $character.initiative = roll("d20")
                        }}>🎲</button
                    >
                {/if}
            </p>
        </td>
    </tr>
</table>

<style>
    input {
        width: 3rem;
    }
</style>
