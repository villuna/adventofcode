module DayEleven

open Core

let parseFile filename =
    readLines filename
    |> List.map Seq.toList
    |> List.map (List.map (fun c -> int c - int '0')) // I love this

let gridIndices n = [for i in 0 .. n - 1 -> [for j in 0 .. n - 1 -> (i, j)]]

let isInBounds size point = fst point >= 0 && snd point >= 0 && fst point < size && snd point < size

let contains lst elem = (lst |> List.filter (fun x -> x = elem) |> List.length) <> 0

let updateGrid (grid: int list list) index =
    let adj = [for i in -1 .. 1 -> [for j in -1 .. 1 do if (i, j) <> (0, 0) then yield (i, j)]]
              |> flatten
              |> List.map (fun t -> (fst t + fst index, snd t + snd index))
              |> List.filter (isInBounds (List.length grid))

    if grid.[fst index].[snd index] >= 10 then
        grid
        |> List.mapi (fun i lst -> lst
                                   |> List.mapi (fun j x -> if (i, j) = index then 0
                                                            else if contains adj (i, j) && grid.[i].[j] <> 0 then
                                                                grid.[i].[j] + 1
                                                            else grid.[i].[j]))
    else grid

let rec flash grid =
    let updatedGrid = gridIndices (List.length grid)
                      |> flatten
                      |> List.fold updateGrid grid

    let numFlashes = updatedGrid |> flatten |> List.filter ((<=) 10) |> List.length

    if numFlashes = 0 then
        updatedGrid
    else
        flash updatedGrid

// Iterates the grid n times and then returns the total number of flashes.
let rec numFlashes n grid =
    let numZeros = flatten >> List.filter ((=) 0) >> List.length

    if n = 0 then
        numZeros grid
    else
        let updatedGrid = flash (grid |> List.map (List.map ((+) 1)))
        (numZeros grid) + numFlashes (n - 1) updatedGrid

let rec findSyncHelper n grid =
    let allZeros = flatten >> List.forall (fun i -> i = 0)

    if allZeros grid then
        n
    else
        let updatedGrid = flash (grid |> List.map (List.map ((+) 1)))
        findSyncHelper (n + 1) updatedGrid

let findSync = findSyncHelper 0

let partOne filename = parseFile filename |> numFlashes 100 |> printfn "%d"

let partTwo filename = parseFile filename |> findSync |> printfn "%d"

let dayEleven filename part =
    match part with
    | 1 -> partOne filename
    | 2 -> partTwo filename
    | _ -> printfn "Invalid part number"
