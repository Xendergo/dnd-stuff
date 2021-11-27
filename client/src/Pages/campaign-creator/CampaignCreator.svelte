<script lang="ts">
    import { CAMPAIGNS } from "../../data"

    let name = ""

    let error = ""
</script>

<main>
    <div class="column fullscreen">
        <input placeholder="Campaign name" bind:value={name} />
        <button
            on:click={() => {
                if (name !== "") {
                    if ($CAMPAIGNS[name] === undefined) {
                        $CAMPAIGNS[name] = []

                        location.href = `../campaign=${name}`
                    } else {
                        error = "You're already GMing a campaign with that name"
                    }
                } else {
                    error = "Enter a name"
                }
            }}>Create</button
        >
        <p style="color: crimson; text-align: center">{error}</p>
    </div>
</main>
