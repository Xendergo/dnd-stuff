<script lang="ts">
    import { CAMPAIGNS, CAMPAIGN_NAME, CHARACTER_NAME, IS_GM } from "../../data"

    import { Store } from "../../better-store"

    import BattleData from "./BattleData.svelte"

    if (CHARACTER_NAME === null) {
        window.location.href = "../"
    }

    if (
        CAMPAIGN_NAME === null ||
        !Object.keys($CAMPAIGNS).includes(CAMPAIGN_NAME)
    ) {
        location.href = "../"
    }

    let characterList = new Store($CAMPAIGNS[CAMPAIGN_NAME], v =>
        CAMPAIGNS.notifySubscribers()
    )

    let characterIndex = $characterList.findIndex(
        v => v.name === CHARACTER_NAME
    )

    if (characterIndex === -1) {
        window.location.href = "../"
    }

    let character = new Store($characterList[characterIndex], v =>
        characterList.notifySubscribers()
    )
</script>

<svelte:head>
    <title>{CHARACTER_NAME} | Character data</title>
</svelte:head>

<main>
    <BattleData bind:character />
</main>
