<script lang="ts">
    import { CAMPAIGNS_GMING, CAMPAIGN_NAME, IS_GM } from "../../data"

    import { characters } from "../../characters"
</script>

<main>
    <table>
        {#if !$IS_GM}
            {#each $characters as character}
                <tr
                    ><td
                        ><a
                            href={`/character-data/?character=${character.name}`}
                        >
                            {character.name}
                        </a></td
                    >
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <a href="/character-creator/"
                        ><button>New character</button></a
                    >
                </td>
            </tr>
        {:else if $CAMPAIGN_NAME === null}
            {#each Object.keys($CAMPAIGNS_GMING) as name}
                <tr>
                    <td>{name}</td>
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <a href="/campaign-creator/"
                        ><button>New campaign</button></a
                    >
                </td>
            </tr>
        {:else}
            <tr><td>{$CAMPAIGN_NAME}</td></tr>
            {#each $CAMPAIGNS_GMING[$CAMPAIGN_NAME] as npc}
                <tr>
                    <td>{npc.name}</td>
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <button>New NPC</button>
                </td>
            </tr>
        {/if}
    </table>

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
    td {
        border-bottom: 1px dashed white;
    }

    td a {
        display: inline-block;
        width: 100%;
        height: 100%;
        padding: 8px;
        text-decoration: none;
    }

    .no-border {
        border-bottom: none;
    }

    table {
        text-align: center;
        margin-left: auto;
        margin-right: auto;
    }

    #gm-button {
        position: fixed;
        right: 16px;
        bottom: 16px;
    }
</style>
