module DaySix

open Core


let counts (initList: int64 list) =
    [0L .. 8L]
    |> List.map (fun i -> List.filter (fun x -> x = i) initList |> List.length)
    |> List.map int64

let update (fishList: int64 list) =
    [0 .. 8]
    |> List.map (fun i -> match i with
                          | 8 -> fishList.[0]
                          | 7 -> fishList.[8]
                          | 6 -> fishList.[0] + fishList.[7]
                          | _ -> fishList.[(i + 1) % 7])

let daySix filename part =
    let mutable fishList = (readLines filename).[0].Split ","
                           |> Array.toList
                           |> List.map int64
                           |> counts

    let count = if part = 1 then 80
                else 256

    for i in 1 .. count do
        fishList <- update fishList

    printfn "%d" (List.sum fishList)
