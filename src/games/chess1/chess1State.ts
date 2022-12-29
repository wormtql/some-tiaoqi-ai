import { Renderable } from "@/render/types"
import { create2dArray } from "@/utils/common"

export const CHESS1_RED = 1
export const CHESS1_BLUE = 2

export class Chess1State {
    public data: Array<Array<number>>
    public size: number

    constructor(size = 9) {
        this.size = size
        this.data = []
        for (let i = 0; i < size; i++) {
            this.data.push(new Array(size))
            for (let j = 0; j < size; j++) {
                this.data[this.data.length - 1][j] = 0
            }
        }

        for (let i = 0; i < 4; i++) {
            for (let j = 0; j < 4; j++) {
                this.data[i][j] = CHESS1_RED
                this.data[size - i - 1][size - j - 1] = CHESS1_BLUE
            }
        }
    }

    public move(fromX: number, fromY: number, toX: number, toY: number): void {
        this.data[toX][toY] = this.data[fromX][fromY]
        this.data[fromX][fromY] = 0
    }

    public getActionsFromSinglePoint(x: number, y: number): [number, number][] {
        const u = [0, 1, 0, -1]
        const v = [1, 0, -1, 0]
        const u2 = [0, 2, 0, -2]
        const v2 = [2, 0, -2, 0]

        const temp: Set<number> = new Set()
        // adjacent
        for (let i = 0; i < 4; i++) {
            const nx = x + u[i]
            const ny = y + v[i]
            if (nx >= 0 && nx < this.size && ny >= 0 && ny < this.size) {
                if (this.data[nx][ny] === 0) {
                    temp.add((nx << 16) | ny)
                }
            }
        }

        // jumps
        const vis: boolean[][] = create2dArray(this.size, this.size, false)
        const queue: [number, number][] = [[x, y]]
        vis[x][y] = true

        while (queue.length > 0) {
            const [px, py] = (queue.pop() as [number, number])
            for (let i = 0; i < 4; i++) {
                const nx2 = px + u2[i]
                const ny2 = py + v2[i]
                const nx1 = px + u[i]
                const ny1 = py + v[i]
                if (nx2 >= 0 && nx2 < this.size && ny2 >= 0 && ny2 < this.size && !vis[nx2][ny2]) {
                    if (this.data[nx2][ny2] === 0 && this.data[nx1][ny1] !== 0) {
                        vis[nx2][ny2] = true
                        temp.add((nx2 << 16) | ny2)
                        queue.push([nx2, ny2])
                    }
                }
            }
        }

        const result: [number, number][] = []
        for (const item of temp) {
            const x = item >> 16
            const y = item & ((1 << 16) - 1)
            result.push([x, y])
        }

        return result
    }
}
