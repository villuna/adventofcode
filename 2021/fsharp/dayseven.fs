module DaySeven

open Core

let median lst =
    let len = List.length lst

    (lst |> List.sort).[len / 2]

let partOne positions =
    let minimum = median positions

    positions
    |> List.map (fun x -> abs (x - minimum))
    |> List.sum

let crabDistance x y =
    let n = abs (x - y)
    (n * (n + 1)) / 2

let partTwo positions =
    // Have to convert between types in order for List.average to work
    let floats = List.map float positions
    let mean = int (List.average floats)

    positions
    |> List.map (crabDistance mean)
    |> List.sum

let daySeven filename part =
    let positions = (readLines filename).[0].Split ","
                    |> Array.toList
                    |> List.map int

    match part with
    | 1 -> partOne positions |> printfn "%d"
    | 2 -> partTwo positions |> printfn "%d"
    | _ -> printfn "invalid question number"

