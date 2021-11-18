<script lang="ts">
    import { CAMPAIGNS_GMING, CAMPAIGN_NAME, IS_GM } from "../../data"

    import { characters } from "../../characters"
</script>

<main>
    <div class="column">
        {#if !$IS_GM}
            {#each $characters as character}
                <p>{character.name}</p>
            {/each}
            <div>
                <button>New character</button>
            </div>
        {:else if $CAMPAIGN_NAME === null}
            {#each Object.keys($CAMPAIGNS_GMING) as name}
                <p>{name}</p>
            {/each}
            <div>
                <a href="/campaign-creator/"><button>New campaign</button></a>
            </div>
        {:else}
            <p>{$CAMPAIGN_NAME}</p>
            {#each $CAMPAIGNS_GMING[$CAMPAIGN_NAME] as npc}
                <p>{npc.name}</p>
            {/each}
            <div>
                <button>New NPC</button>
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
    p {
        border-bottom: 1px dotted white;
    }

    #gm-button {
        position: fixed;
        right: 16px;
        bottom: 16px;
    }
</style>
