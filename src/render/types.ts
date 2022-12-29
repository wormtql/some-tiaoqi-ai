export interface Renderable {
    render(ctx: CanvasRenderingContext2D): void
}

export class GameObject implements Renderable {
    protected eventListeners: Map<string, any[]> = new Map()

    public render(ctx: CanvasRenderingContext2D): void {
        return
    }

    public onMousedown(x: number, y: number, e: MouseEvent): void {
        return
    }

    public onMousemove(x: number, y: number, e: MouseEvent): void {
        return
    }

    public onMouseleave(x: number, y: number, e: MouseEvent): void {
        return
    }

    public addEventListener(name: string, handler: any): void {
        let handlers = this.eventListeners.get(name)
        if (handlers === undefined) {
            handlers = []
            this.eventListeners.set(name, handlers)
        }

        handlers = handlers as any[]
        handlers.push(handler)
    }

    public emitEvent(name: string, ...args: any[]): void {
        const handlers = this.eventListeners.get(name)
        if (handlers) {
            for (const item of handlers) {
                item(...args)
            }
        }
    }
}
