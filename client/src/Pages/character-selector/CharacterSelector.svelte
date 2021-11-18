<script lang="ts">
    import { GAMES_GMING, GAME_NAME, IS_GM } from "../../data"

    import { characters } from "../../characters"
</script>

<main>
    <div>
        {#if !$IS_GM}
            {#each $characters as character}
                <p>{character.name}</p>
            {/each}
            <div>
                <button>New character</button>
            </div>
        {:else if $GAME_NAME === null}
            {#each Object.keys($GAMES_GMING) as name}
                <p>{name}</p>
            {/each}
            <div>
                <button>New game</button>
            </div>
        {:else}
            {#each $GAMES_GMING[$GAME_NAME] as npc}
                <p>{npc.name}</p>
            {/each}
            <div>
                <button>New character</button>
            </div>
        {/if}
    </div>

    {#if !$IS_GM}
        <button id="gm-button" on:click={() => ($IS_GM = true)}
            >I'm the GM</button
        >
    {:else}
        <button id="gm-button" on:click={() => ($IS_GM = false)}
            >I'm not the GM</button
        >
    {/if}
</main>

<style>
    div {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    p {
        border-bottom: 1px dotted white;
    }

    #gm-button {
        position: fixed;
        right: 16px;
        bottom: 16px;
    }
</style>
