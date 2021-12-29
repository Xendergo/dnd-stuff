<script lang="ts">
    import type { Store } from "src/better-store"
    import type { Character } from "src/characters"

    export let character: Store<Character>

    function dragOver() {
        hoveringCastButton = true
        notHoveringCastButton = false
        dropped = false
    }

    function dragLeave() {
        hoveringCastButton = false
        notHoveringCastButton = true
    }

    function drop(e) {
        dropped = true

        const index = e.dataTransfer.getData("text/plain")

        if (index !== null) {
            $character.currentSpellSlots[parseInt(index)]--
        }
    }

    let hoveringCastButton = false
    let notHoveringCastButton = false
    let dropped = false
</script>

<span
    on:dragover|preventDefault={dragOver}
    on:dragleave={dragLeave}
    on:drop|preventDefault={drop}
    class:hoveringCastButton
    class:notHoveringCastButton
    class:dropped
    class="circle">🪄</span
>

<style>
    .hoveringCastButton {
        animation: startHovering 0.5s;
        animation-fill-mode: forwards;
    }

    .notHoveringCastButton {
        animation: endHovering 0.5s;
    }

    .dropped {
        animation: dropped 0.6s;
    }

    @keyframes startHovering {
        to {
            box-shadow: 0 0 8px 4px rgb(255, 255, 128);
            transform: scale(1.2);
            animation-play-state: paused;
        }
    }

    @keyframes endHovering {
        from {
            box-shadow: 0 0 8px 4px rgb(255, 255, 128);
            transform: scale(1.2);
        }

        to {
            box-shadow: 0 0 0px 0px rgb(255, 255, 128);
            transform: scale(1);
        }
    }

    @keyframes dropped {
        from {
            box-shadow: 0 0 8px 4px rgb(255, 255, 128);
            transform: scale(1.2);
        }

        70% {
            transform: scale(1);
        }

        to {
            box-shadow: 0 0 300px 8px rgb(0, 0, 0);
        }
    }
</style>
