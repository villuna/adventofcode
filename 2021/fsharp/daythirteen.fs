module DayThirteen

open Core

type Grid = (int * int) list

type Fold =
    | Horizontal of int
    | Vertical of int

let lineToFold (line: string) =
    // coord will be a string containing 'x=_' or 'y=_'
    let coord = (line.Split ' ' |> Array.toList).[2].Split '=' |> Array.toList

    match coord.[0] with
        | "x" -> Horizontal (int coord.[1])
        | "y" -> Vertical (int coord.[1])

let foldPoint fold point =
    match fold with
        | Horizontal x -> if fst point < x then point
                          else (2 * x - (fst point), snd point)
        | Vertical y ->   if snd point < y then point
                          else (fst point, 2 * y - (snd point))

let applyFold grid fold =
    let folded = List.map (foldPoint fold) grid

    folded |> List.countBy id |> List.map fst

let parseGrid filename =
    let lines = readLines filename

    let grid = lines |> List.filter (fun s -> not (s.StartsWith("fold")) && s <> "")
                     |> List.map (fun s -> s.Split "," |> Array.toList)
                     |> List.map (fun l -> (int l.[0], int l.[1]))

    let folds = lines |> List.filter (fun s -> s.StartsWith("fold"))
                      |> List.map lineToFold

    grid, folds

let displayGrid grid =
    let maxX = grid |> List.map fst |> List.max
    let maxY = grid |> List.map snd |> List.max

    [for i in 0 .. maxY -> [for j in 0 .. maxX -> if contains (j, i) grid then "#" else "."]]
    |> List.iter (fun lst -> printfn "%s" (String.concat "" lst))

let partOne filename =
    let grid, folds = parseGrid filename

    applyFold grid folds.[0] |> List.length |> printfn "The answer is %d"

let partTwo filename =
    let grid, folds = parseGrid filename

    folds |> List.fold applyFold grid |> displayGrid

let dayThirteen filename part =
    match part with
        | 1 -> partOne filename
        | 2 -> partTwo filename
        | _ -> printfn "Invalid part number"
