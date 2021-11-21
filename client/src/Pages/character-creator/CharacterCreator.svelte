<script lang="ts">
    import { CAMPAIGNS_GMING, urlParams } from "../../data"

    import { Character, characters } from "../../characters"

    let IS_GM = urlParams.get("gm") === "true"
    let CAMPAIGN_NAME = urlParams.get("campaign")

    if (
        IS_GM &&
        (CAMPAIGN_NAME === null ||
            !Object.keys($CAMPAIGNS_GMING).includes(CAMPAIGN_NAME))
    ) {
        IS_GM = false
    }

    let name = ""

    let error = ""
</script>

<main>
    <div class="column fullscreen">
        <input
            placeholder={`Name of ${IS_GM ? "NPC" : "Character"}`}
            bind:value={name}
        />
        <button
            on:click={() => {
                if (name !== "") {
                    if (IS_GM) {
                        if (
                            $CAMPAIGNS_GMING[CAMPAIGN_NAME].reduce(
                                (a, v) => v.name === name || a,
                                false
                            )
                        ) {
                            error =
                                "You're already playing an NPC with that name"
                        } else {
                            CAMPAIGNS_GMING.update(
                                v => (
                                    v[CAMPAIGN_NAME].push(
                                        new Character({ name })
                                    ),
                                    v
                                )
                            )

                            location.href = `../character-data/?gm=true&campaign=${CAMPAIGN_NAME}&character=${name}`
                        }
                    } else {
                        if (
                            $characters.reduce(
                                (a, v) => v.name === name || a,
                                false
                            )
                        ) {
                            error =
                                "You're already playing a character with that name"
                        } else {
                            characters.update(
                                v => (v.push(new Character({ name })), v)
                            )

                            location.href = `../character-data/?gm=false&character=${name}`
                        }
                    }
                } else {
                    error = "Enter a name"
                }
            }}>Create</button
        >
        <p style="color: crimson; text-align: center">{error}</p>
    </div>
</main>
