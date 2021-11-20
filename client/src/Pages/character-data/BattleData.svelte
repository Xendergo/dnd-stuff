<script lang="ts">
    import type { Character } from "src/characters"
    import { roll } from "../../dice"

    export let character: Character
</script>

<table>
    <tr>
        <td>
            <p>
                HP <input type="number" bind:value={character.hp} /><button
                    on:click={() => (character.hp -= 1)}>-1</button
                ><button on:click={() => (character.hp += 1)}>+1</button><button
                    on:click={() => (character.hp = character.hp_max)}
                    >🔄</button
                >
            </p>
        </td>
    </tr>
    <tr>
        <td>
            <p>
                max HP <input type="number" bind:value={character.hp_max} />
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
                    bind:value={character.initiative}
                /><button
                    on:click={() => {
                        character.initiative = roll("d20")
                    }}>🎲</button
                >
            </p>
        </td>
    </tr>
</table>

<style>
    input {
        width: 3rem;
    }
</style>
