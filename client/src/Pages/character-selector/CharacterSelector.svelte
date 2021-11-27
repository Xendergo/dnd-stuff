<script lang="ts">
    import { CAMPAIGNS_GMING, IP_ADDRESS, urlParams } from "../../data"

    import { characters } from "../../characters"
    import { connect } from "../../server-interface"

    let IS_GM = urlParams.get("gm") === "true"
    let CAMPAIGN_NAME: string | null = urlParams.get("campaign") ?? null

    let tmp_ip_address = ""
</script>

<main>
    <table>
        {#if !IS_GM}
            {#each $characters as character}
                <tr
                    ><td
                        ><a
                            href={`./character-data/?character=${character.name}`}
                        >
                            {character.name}
                        </a></td
                    >
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <a href="./character-creator/"
                        ><button>New character</button></a
                    >
                </td>
            </tr>
        {:else if CAMPAIGN_NAME === null}
            {#each Object.keys($CAMPAIGNS_GMING) as name}
                <tr>
                    <td><a href={`./?gm=true&campaign=${name}`}>{name}</a></td>
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <a href="./campaign-creator/"
                        ><button>New campaign</button></a
                    >
                </td>
            </tr>
        {:else}
            <tr><td class="title">{CAMPAIGN_NAME}</td></tr>
            {#each $CAMPAIGNS_GMING[CAMPAIGN_NAME] as npc}
                <tr>
                    <td
                        ><a
                            href={`./character-data/?gm=true&campaign=${CAMPAIGN_NAME}&character=${npc.name}`}
                            >{npc.name}</a
                        ></td
                    >
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <a
                        href={`./character-creator/?gm=true&campaign=${CAMPAIGN_NAME}`}
                        ><button>New NPC</button></a
                    >
                </td>
            </tr>
        {/if}
    </table>

    {#if !IS_GM}
        <button id="gm-button" on:click={() => (IS_GM = true)}
            >I'm the GM</button
        >
    {:else}
        <button id="gm-button" on:click={() => (IS_GM = false)}
            >I'm not the GM</button
        >
    {/if}

    {#if $IP_ADDRESS === null}
        <input placeholder="Game IP" bind:value={tmp_ip_address} />
        <button
            on:click={() => {
                $IP_ADDRESS = tmp_ip_address
                connect()
            }}>Join</button
        >
    {/if}
</main>

<style>
    td {
        border-bottom: 1px dashed white;
        padding: 8px;
    }

    .title {
        font-size: xx-large;
        border-bottom: 1px solid white;
    }

    td a {
        display: inline-block;
        width: 100%;
        height: 100%;
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
