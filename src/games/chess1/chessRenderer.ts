import { GameObject, Renderable } from "@/render/types";
import { Chess1State } from "./chess1State";

export class Chess1Renderer extends GameObject {
    public state: Chess1State

    public gridSize = 52
    public redColor = "red"
    public blueColor = "blue"
    public emptyColor = "rgba(0, 0, 0, 0.05)"
    public hoverColor = "rgba(0, 0, 0, 0.1)"
    public gap = 8

    private hoverCoord: [number, number] | null = null
    private clickState: [number, number] | null = null
    private viableMoves: [number, number][] = []
    private lastAction: [number, number, number, number] | null = null

    private restrictPlayer: 0 | 1 | 2 = 0
    private locked = false

    public constructor(state: Chess1State) {
        super()
        this.state = state
    }

    public setRestricePlayer(player: 0 | 1 | 2): void {
        this.restrictPlayer = player
    }

    public render(ctx: CanvasRenderingContext2D): void {
        const size = this.state.size
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                const boardX = j
                const boardY = size - i - 1
                const grid: number = this.state.data[boardX][boardY]
                if (grid == 0) {
                    ctx.fillStyle = this.emptyColor
                } else if (grid == 1) {
                    ctx.fillStyle = this.redColor
                } else {
                    ctx.fillStyle = this.blueColor
                }

                ctx.beginPath()
                ctx.arc(
                    j * (this.gridSize + this.gap) + this.gridSize / 2,
                    i * (this.gridSize + this.gap) + this.gridSize / 2,
                    this.gridSize / 2,
                    0, 2 * Math.PI, false
                )
                ctx.fill()

                // hover state
                if (this.hoverCoord && boardX === this.hoverCoord[0] && boardY === this.hoverCoord[1]) {
                    ctx.fillStyle = this.hoverColor
                    ctx.fillRect(j * (this.gridSize + this.gap), i * (this.gridSize + this.gap), this.gridSize, this.gridSize)
                }

                // clicked state
                if (this.clickState && boardX === this.clickState[0] && boardY === this.clickState[1]) {
                    ctx.fillStyle = this.hoverColor
                    ctx.fillRect(j * (this.gridSize + this.gap), i * (this.gridSize + this.gap), this.gridSize, this.gridSize)
                    ctx.fillStyle = "yellow"
                    ctx.fillRect(
                        j * (this.gridSize + this.gap) + (this.gridSize - 10) / 2,
                        i * (this.gridSize + this.gap) + (this.gridSize - 10) / 2,
                        10,
                        10
                    )

                    // viable moves
                    for (const mov of this.viableMoves) {
                        const x = mov[0]
                        const y = this.state.size - mov[1] - 1

                        ctx.fillStyle = "rgba(0, 255, 0, 0.1)"
                        ctx.fillRect(x * (this.gridSize + this.gap), y * (this.gridSize + this.gap), this.gridSize, this.gridSize)
                        ctx.fillStyle = "green"
                        ctx.fillRect(
                            x * (this.gridSize + this.gap) + (this.gridSize - 10) / 2,
                            y * (this.gridSize + this.gap) + (this.gridSize - 10) / 2,
                            10,
                            10
                        )
                    }
                }
                
                // last move state
                if (this.lastAction &&
                    (this.lastAction[0] === boardX && this.lastAction[1] === boardY
                        || this.lastAction[2] === boardX && this.lastAction[3] === boardY)) {
                    ctx.fillStyle = "black"
                    ctx.fillRect(
                        j * (this.gridSize + this.gap) + (this.gridSize - 10) / 2,
                        i * (this.gridSize + this.gap) + (this.gridSize - 10) / 2,
                        10,
                        10
                    )
                }
            }
        }
    }

    private getGridCoordinate(n: number): number {
        return Math.floor(n / (this.gridSize + this.gap))
    }

    private getBoardCoordinate(offsetX: number, offsetY: number): [number, number] | null {
        const gridX = this.getGridCoordinate(offsetX)
        const gridY = this.getGridCoordinate(offsetY)

        if (offsetX <= gridX * (this.gridSize + this.gap) + this.gridSize && offsetY <= gridY * (this.gridSize + this.gap) + this.gridSize) {
            return [gridX, this.state.size - gridY - 1]
        } else {
            return null
        }
    }

    public onMousemove(x: number, y: number, e: MouseEvent): void {
        const boardCoord = this.getBoardCoordinate(x, y)
        if (boardCoord) {
            const [boardX, boardY] = boardCoord
            // const cell = this.state.data[boardX][boardY]
            // if (cell !== 0) {
                this.hoverCoord = boardCoord
            // }
            
        }
    }

    public onMousedown(x: number, y: number, e: MouseEvent): void {
        if (this.locked) {
            return
        }

        const boardCoord = this.getBoardCoordinate(x, y)
        if (boardCoord) {
            const [boardX, boardY] = boardCoord

            // if is the same position, then cancel selected
            if (this.clickState && this.clickState[0] === boardX && this.clickState[1] === boardY) {
                this.clickState = null
                return
            }
            
            // if is a valid target position, then move
            if (this.clickState) {
                for (const item of this.viableMoves) {
                    if (item[0] === boardX && item[1] === boardY) {
                        this.state.move(this.clickState[0], this.clickState[1], boardX, boardY)
                        this.setLastAction(this.clickState[0], this.clickState[1], boardX, boardY)
                        this.emitEvent("move", this.clickState[0], this.clickState[1], boardX, boardY)
                        this.clickState = null
                        return
                    }
                }
            }
            

            const cell = this.state.data[boardX][boardY]
            if ((this.restrictPlayer === 0 && cell !== 0) || (this.restrictPlayer !== 0 && cell === this.restrictPlayer)) {
                this.clickState = boardCoord
                const actions = this.state.getActionsFromSinglePoint(boardX, boardY)
                this.viableMoves = actions
                // console.log(actions)
            }
        }
        
    }

    public onMouseleave(x: number, y: number, e: MouseEvent): void {
        this.hoverCoord = null
    }

    public setLastAction(a: number, b: number, c: number, d: number): void {
        this.lastAction = [a, b, c, d]
    }

    public clearLastAction(): void {
        this.lastAction = null
    }

    public lock(): void {
        this.locked = true
    }

    public unlock(): void {
        this.locked = false
    }
}
