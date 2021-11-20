<script lang="ts">
    import { characters } from "../../characters"
    import BattleData from "./BattleData.svelte"

    const urlParams = new URLSearchParams(window.location.search)

    let character_name = urlParams.get("character")

    if (character_name === null) {
        window.location.href = "/"
    }

    let character_index = $characters.findIndex(v => v.name === character_name)

    if (character_index === -1) {
        window.location.href = "/"
    }

    let character = $characters[character_index]
</script>

<svelte:head>
    <title>{character_name} | Character data</title>
</svelte:head>

<main>
    <BattleData bind:character />
</main>
