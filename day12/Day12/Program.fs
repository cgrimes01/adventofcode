// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions
open System.IO

let getNumbers (numbersString:string) = 
    Regex.Split(numbersString, "[^0-9-]+")
    |> Seq.filter(fun i -> i.Length > 0)
    |> Seq.map(fun i -> int i)

let addNumbers (numbers:seq<int>) =
    numbers
    |> Seq.sum

let sumAllNumbers (input:string) =
    addNumbers (getNumbers input)

let fileInputPart1 = String.Concat (File.ReadAllLines @"D:\AdventOfCode\adventofcode2015\day12\day12input.txt")

[<EntryPoint>]
let main argv =
    printfn "PART 1:"
    printfn "%i" (sumAllNumbers fileInputPart1)
    0 // return an integer exit code
