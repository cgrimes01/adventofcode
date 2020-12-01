// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions
open System.IO

let removeChar a (list:string) = 
    list 
    |> Seq.filter (fun x -> x <> a)
    |> String.Concat

let replaceWithSingle pattern input = 
    Regex.Replace(input, pattern, "_")

let part1CleanUpString input =
    replaceWithSingle """(\\\\)""" input
    |> replaceWithSingle """(\\")"""
    |> replaceWithSingle """(\\x\w\w)"""
    |> removeChar '"'

let part2Encode input =
    Regex.Replace(input, """\\""", """\\""")
    |> fun input -> Regex.Replace(input, "\"", "\\\"")

let part2 (input:string[]) = 
    input
    |> Seq.map (fun x -> ("\"" + (part2Encode x) + "\""))
    |> String.Concat

let outputResults (codeLength:int) (stringLength:int) (solution:int) = 
    printfn "Characters of code =  %i\nString length =  %i\nSolution = %i" codeLength stringLength solution

let fileInputPart1 = String.Concat (File.ReadAllLines @"D:\AdventOfCode\adventofcode2015\day8\day8\day8input.txt")
let fileInputPart2 = File.ReadAllLines @"D:\AdventOfCode\adventofcode2015\day8\day8\day8input.txt"

[<EntryPoint>]
let main argv =
    printfn "PART 1:"
    outputResults fileInputPart1.Length (part1CleanUpString fileInputPart1).Length (fileInputPart1.Length - (part1CleanUpString fileInputPart1).Length)
    printfn "\nPART 2:"
    outputResults (String.Concat fileInputPart2).Length (part2 fileInputPart2).Length ((part2 fileInputPart2).Length  - (String.Concat fileInputPart2).Length)
    0 // return an integer exit code


