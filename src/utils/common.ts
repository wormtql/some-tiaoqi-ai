export function create2dArray<T>(m: number, n: number, init: T): T[][] {
    const temp = new Array(m)
    for (let i = 0; i < m; i++) {
        temp[i] = new Array(n)
        for (let j = 0; j < n; j++) {
            temp[i][j] = init
        }
    }
    return temp
}
