<script lang="ts">
    import { CAMPAIGNS, CAMPAIGN_NAME, IP_ADDRESS, IS_GM } from "../../data"

    import { connect, getCharacters } from "../../server-interface"
    import BattleData from "../character-data/BattleData.svelte"

    let tmp_ip_address = ""
</script>

<main>
    <table>
        {#if CAMPAIGN_NAME === null}
            {#each Object.keys($CAMPAIGNS) as name}
                <tr>
                    <td><a href={`./?campaign=${name}`}>{name}</a></td>
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
            {#each getCharacters() as character}
                <tr>
                    <td
                        ><a
                            href={`./character-data/?campaign=${CAMPAIGN_NAME}&character=${character.value.name}`}
                            >{character.value.name}</a
                        ></td
                    >
                </tr>
            {/each}
            <tr>
                <td class="no-border">
                    <a href={`./character-creator/?campaign=${CAMPAIGN_NAME}`}
                        ><button>New character</button></a
                    >
                </td>
            </tr>
        {/if}
    </table>

    <div id="bottom-row">
        {#if $IP_ADDRESS === null}
            <input
                id="ip-input"
                placeholder="Game IP"
                bind:value={tmp_ip_address}
            />
            <button
                id="join-button"
                on:click={() => {
                    $IP_ADDRESS = tmp_ip_address
                    connect()
                }}>Join</button
            >
        {/if}

        {#if !$IS_GM}
            <button id="gm-button" on:click={() => ($IS_GM = true)}
                >I'm the GM</button
            >
        {:else}
            <button id="gm-button" on:click={() => ($IS_GM = false)}
                >I'm not the GM</button
            >
        {/if}
    </div>
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

    #bottom-row {
        position: fixed;
        left: 16px;
        right: 16px;
        bottom: 16px;

        text-align: center;
    }

    #ip-input {
        float: left;
    }

    #join-button {
        float: left;
    }

    #gm-button {
        float: right;
    }
</style>
