module Core

(*
A collection of functions that aren't necessarily tied to one solution
or just functions I thought were cool.
*)

let readLines filename = Seq.toList (System.IO.File.ReadLines(filename))

// Shamelessly stolen from stackoverflow
let modulo n m = ((n % m) + m) % m

let rec unorderedPairs (lst: 'a list) =
    match lst with
    | [_] | [] -> []
    | x::xs -> (List.map (fun y -> (x, y)) xs) @ (unorderedPairs xs)

let flatten lst = List.fold (@) [] lst

let contains item lst = (List.filter (fun elem -> elem = item) lst |> List.length) <> 0
