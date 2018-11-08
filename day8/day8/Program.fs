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

let cleanUpString input =
    replaceWithSingle """(\\\\)""" input
    |> replaceWithSingle """(\\")"""
    |> replaceWithSingle """(\\x\w\w)"""
    |> removeChar '"'

let fileInput = String.Concat (File.ReadAllLines @"D:\AdventOfCode\adventofcode2015\day8\day8\day8input.txt")
let charactersOfCode = fileInput.Length
let stringLength = (cleanUpString fileInput).Length
let solution = charactersOfCode - stringLength

[<EntryPoint>]
let main argv =
    printfn "Characters of code =  %i\nString length =  %i\nSolution = %i" charactersOfCode stringLength solution
    0 // return an integer exit code


