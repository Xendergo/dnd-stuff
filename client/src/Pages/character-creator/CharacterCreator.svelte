<script lang="ts">
    import { CAMPAIGNS, CAMPAIGN_NAME, IS_GM } from "../../data"

    import { Character } from "../../characters"

    if (
        CAMPAIGN_NAME === null ||
        !Object.keys($CAMPAIGNS).includes(CAMPAIGN_NAME)
    ) {
        IS_GM.value = false
    }

    let name = ""

    let error = ""
</script>

<main>
    <div class="column fullscreen">
        <input placeholder={`Name of character`} bind:value={name} />
        <button
            on:click={() => {
                if (name !== "") {
                    if (
                        $CAMPAIGNS[CAMPAIGN_NAME].reduce(
                            (a, v) => v.name === name || a,
                            false
                        )
                    ) {
                        error =
                            "You're already playing a character with that name"
                    } else {
                        CAMPAIGNS.update(
                            v => (
                                v[CAMPAIGN_NAME].push(new Character({ name })),
                                v
                            )
                        )

                        location.href = `../character-data/campaign=${CAMPAIGN_NAME}&character=${name}`
                    }
                } else {
                    error = "Enter a name"
                }
            }}>Create</button
        >
        <p style="color: crimson; text-align: center">{error}</p>
    </div>
</main>
