// Learn more about F# at http://fsharp.org

open System

let part1Input ="3113322113"

let rec matchLength (input:string) =
    match input.Length with
        | 0 -> 0
        | 1 -> 1 
        | _ -> if (input.[0] = input.[1]) then 1 + matchLength input.[1..] else 1

let rec lookAndSay (input:string) = 
    match input.Length with
        | 0 -> ""
        | _ -> matchLength input |> fun x -> sprintf "%i%c%s" x input.[0] (lookAndSay input.[x..])
  
let getLengthAfterN n =
    [1..n]
    |> Seq.fold (fun input _ -> lookAndSay input) part1Input
    //|> Seq.length

[<EntryPoint>]
let main argv =
    //printfn "Length of part 1: %i" (getLengthAfterN 1)
    printfn "Length of part 1: %s" (getLengthAfterN 40)
    0 // return an integer exit code
