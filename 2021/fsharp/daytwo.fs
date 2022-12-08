module DayTwo

open Core

[<Struct>]
type Position = {
    depth: int
    distance: int
    aim: int
}

let applyCommand position command =
    match command with
    | ("up", i) -> { position with aim = position.aim - i }
    | ("down", i) -> { position with aim = position.aim + i }
    | ("forward", i) -> { position with depth = position.depth + position.aim * i
                                        distance = position.distance + i }
    | _ -> position

let runCommands filename =
    let lines = readLines filename
    let initialPosition = { depth = 0; distance = 0; aim = 0 }

    lines 
    |> List.map (fun s -> s.Split ' ')
    |> List.map (fun sl -> (sl.[0], (int sl.[1])))
    |> List.fold applyCommand initialPosition

let partOne filename =
    let finalPos = runCommands filename
    finalPos.aim * finalPos.distance

let partTwo filename =
    let finalPos = runCommands filename
    finalPos.depth * finalPos.distance

let functions = [partOne; partTwo]

let dayTwo filename part =
    match part with
    | 1 | 2 -> functions.[part - 1] filename |> printfn "%d"
    | _ -> printfn "invalid question number"
