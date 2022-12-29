<template>
    <div class="root">
        <canvas id="mainCanvas" width="532" height="532"
            @mousemove="handleMousemove"
            @mousedown="handleMousedown"
            @mouseleave="handleMouseleave"
        ></canvas>
    </div>
</template>

<script lang="ts" setup>
import { onMounted } from "vue"
import { Chess1State } from "@/games/chess1/chess1State"
import { Chess1Renderer } from "./games/chess1/chessRenderer";
import { invoke } from "@tauri-apps/api"

onMounted(() => {
    const canvas = document.getElementById("mainCanvas") as HTMLCanvasElement
    const ctx = canvas.getContext("2d")
    // console.log(ctx)
    if (!ctx) {
        return
    }
    requestAnimationFrame(() => render(ctx))
})

let gameState: Chess1State = new Chess1State(9)
let gameRenderer: Chess1Renderer = new Chess1Renderer(gameState)
gameRenderer.setRestricePlayer(2)

gameRenderer.addEventListener("move", () => {
    gameRenderer.lock()
    invoke("chess1_solve", {
        board: {
            data: gameState.data,
            size: gameState.size
        },
        nextPlayer: 1
    }).then((action: any) => {
        gameRenderer.unlock()
        const a = action["from_x"]
        const b = action["from_y"]
        const c = action["to_x"]
        const d = action["to_y"]
        gameState.move(a, b, c, d)
        gameRenderer.setLastAction(a, b, c, d)
        console.log(action)
    })
})

function render(ctx: CanvasRenderingContext2D) {
    ctx.clearRect(0, 0, 5000, 5000)
    gameRenderer.render(ctx)

    requestAnimationFrame(() => render(ctx))
}

// handle mouse events
function handleMousemove(e: MouseEvent) {
    gameRenderer.onMousemove(e.offsetX, e.offsetY, e)
}

function handleMousedown(e: MouseEvent) {
    gameRenderer.onMousedown(e.offsetX, e.offsetY, e)
}

function handleMouseleave(e: MouseEvent) {
    gameRenderer.onMouseleave(e. offsetX, e.offsetY, e)
}

</script>

<style lang="scss">
body {
    margin: 0;
    padding: 0;
}

.root {
    height: 100vh;
    width: 100vw;

    display: flex;
    justify-content: center;
    align-items: center;
}
</style>
