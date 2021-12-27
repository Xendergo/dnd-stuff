<script lang="ts">
    import {
        CAMPAIGNS,
        CAMPAIGN_NAME,
        CHARACTER_NAME,
        CLIENT_ID,
    } from "../../data"

    import BattleData from "./BattleData.svelte"
    import { characterList } from "../../server-interface"
    import Items from "./Items.svelte"
    import Spells from "./Spells.svelte"

    if (CHARACTER_NAME === null) {
        window.location.href = "../"
    }

    if (
        CAMPAIGN_NAME === null ||
        !Object.keys($CAMPAIGNS).includes(CAMPAIGN_NAME)
    ) {
        location.href = "../"
    }

    let characterIndex = characterList.value!.findIndex(
        v => v.value.name === CHARACTER_NAME
    )

    if (characterIndex === -1) {
        window.location.href = "../"
    }

    let character = characterList.value![characterIndex]

    let writeable =
        character.value.owner === null ||
        character.value.owner === CLIENT_ID.value
</script>

<svelte:head>
    <title>{CHARACTER_NAME} | Character data</title>
</svelte:head>

<main>
    <div>
        <BattleData bind:character {writeable} />
    </div>
    <div>
        <Items bind:character {writeable} />
    </div>
    <div>
        <Spells bind:character {writeable} />
    </div>
</main>

<style>
    main {
        display: flex;
        position: absolute;
        top: 0;
        left: 0;
        height: calc(100vh - (100vh - 100%));
    }

    div + div {
        margin-left: calc(2vw - 16px);
        padding-left: 2vw;
        border-left: 1px dashed grey;
    }

    div {
        margin: 16px;
        min-width: max-content;
    }
</style>
