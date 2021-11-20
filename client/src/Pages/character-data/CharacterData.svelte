<script lang="ts">
    import { CAMPAIGNS_GMING } from "../../data"

    import { Store } from "../../better-store"

    import { characters } from "../../characters"
    import BattleData from "./BattleData.svelte"

    const urlParams = new URLSearchParams(window.location.search)

    let characterName = urlParams.get("character")

    if (characterName === null) {
        window.location.href = "../"
    }

    let IS_GM = urlParams.get("gm") === "true"
    let CAMPAIGN_NAME = urlParams.get("campaign")

    if (
        IS_GM &&
        (CAMPAIGN_NAME === null ||
            !Object.keys($CAMPAIGNS_GMING).includes(CAMPAIGN_NAME))
    ) {
        location.href = "../"
    }

    let characterList = IS_GM
        ? new Store($CAMPAIGNS_GMING[CAMPAIGN_NAME], v =>
              CAMPAIGNS_GMING.notifySubscribers()
          )
        : characters

    let characterIndex = $characterList.findIndex(v => v.name === characterName)

    if (characterIndex === -1) {
        window.location.href = "../"
    }

    let character = new Store($characterList[characterIndex], v =>
        characterList.notifySubscribers()
    )
</script>

<svelte:head>
    <title>{characterName} | Character data</title>
</svelte:head>

<main>
    <BattleData bind:character />
</main>
