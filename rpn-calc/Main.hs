module Main where

import Lib

-- | Main REPL for the program. We treat each line as a separate "program".
-- We do not keep "state" across the different lines.
main :: IO ()
main = interact $ unlines . map processLine . lines
