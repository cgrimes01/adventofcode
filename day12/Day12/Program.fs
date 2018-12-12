// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions
open System.IO
open FSharp.Data

let getNumbers (numbersString:string) = 
    Regex.Split(numbersString, "[^0-9-]+")
    |> Seq.filter(fun i -> i.Length > 0)
    |> Seq.map(fun i -> int i)

let addNumbers (numbers:seq<int>) =
    numbers
    |> Seq.sum

let sumAllNumbers (input:string) =
    addNumbers (getNumbers input)

let rec cleanObject (input:JsonValue) =
    match input.AsArray().Length with
    | 0 -> ""
    | _ -> if input.TryGetProperty("red").IsNone then "" else [ for v in input -> cleanObject v] |> List.toArray |> string

let fileInputPart1 = String.Concat (File.ReadAllLines @"D:\AdventOfCode\adventofcode2015\day12\day12input.txt")
let jsonInput = JsonValue.Parse(fileInputPart1)

[<EntryPoint>]
let main argv =
    printfn "PART 1:"
    printfn "%i" (sumAllNumbers fileInputPart1)
    printfn "PART 2:"
    printfn "%s" (cleanObject (JsonValue.Parse("""{"red": 1}""")))
    0 // return an integer exit code
