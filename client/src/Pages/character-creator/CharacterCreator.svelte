<script lang="ts">
    import { Character, characters } from "../../characters"

    import { IS_GM } from "../../data"

    let name = ""

    let error = ""
</script>

<main>
    <div class="column fullscreen">
        <input placeholder="Name" bind:value={name} />
        <button
            on:click={() => {
                if (name !== "") {
                    if ($IS_GM) {
                        // TODO
                        throw new Error("TODO")
                    } else {
                        characters.update(
                            v => (v.push(new Character({ name })), v)
                        )
                    }
                } else {
                    error = "Enter a name"
                }
            }}>Create</button
        >
        <p style="color: crimson; text-align: center">{error}</p>
    </div>
</main>
