module Lib
    ( processLine
    , processTokens
    ) where

import Text.Read (readMaybe)
import Control.Monad.State.Lazy (State, get, put, evalState)




-- | Process a line of input, and produce a string output
--
processLine :: String -> String
processLine line = unwords $ map show $ evalState (processTokens $ words line) (False, [])


-- | Our working stack is composed of either floats or errors (as strings)
-- [Either String Float]
type Stack = [Either String Float]
-- | The program state consists of the program stack,
-- as well as an indicator, if we should parse subsequent tokens,
-- the boolean flag indicates "comment until the end of the line"
-- therefore: 20 30 + " this is the comment
-- represents a valid program, that will finish with 50 on the stack.
type ProgState = State (Bool, Stack) Stack


-- | Process tokens (list of words) and update the program state.
--
-- This is the main processing routine. Note, that here, we do it "line by line" only.
-- The line is parsed into "tokens", that is, words, simply with `words` function.
-- therefore, you need to separate each individual token by space.
processTokens :: [String] -> ProgState

processTokens [] = do
  (_, stack) <- get
  return stack

processTokens (t:ts) = do
  (ignore, stack) <- get
  if t == "\"" then put (not ignore, stack) >> processTokens ts
               else if not ignore then (case t of
                                      "*" -> opMult
                                      "+" -> opAdd
                                      "pop" -> opPop
                                      _ -> opNum t) >> processTokens ts
                                  else processTokens ts


-- | Multiplication. Represents "*" function.
opMult :: ProgState
opMult = do
  (ignore, stack) <- get
  let new_stack = (if length stack < 2 
                    then do
                        Left "Not enough arguments for *" : stack
                    else do 
                        let a:b:rest = stack
                        ((*) <$> a <*> b) : rest)
  put (ignore, new_stack)
  return new_stack

-- | Addition. Represents "+" function.
opAdd :: ProgState
opAdd = do
  (ignore, stack) <- get
  let new_stack = (if length stack < 2 
                      then do
                          Left "Not enough arguments for +" : stack
                      else do 
                          let a:b:rest = stack
                          ((+) <$> a <*> b) : rest)
  put (ignore, new_stack)
  return new_stack

-- | Pop. Pops the top element from the stack.
-- Note, you can pop an error from the stack to clean it up!
opPop :: ProgState
opPop = do
    (ignore, stack) <- get
    let new_stack = if length stack == 0 
                      then Left "Not enough arguments for pop" : stack
                      else tail stack
    put (ignore, new_stack)
    return new_stack

-- | Numerical literals. It represents a number.
-- We will parse the number or fail. 
-- We treat all numbers as Floats here.
opNum :: String -> ProgState
opNum token = do
    (ignore, stack) <- get
    let new_stack = case (readMaybe token :: Maybe Float) of
                        Nothing -> Left ("Parsing error, expected a number, got: " ++ token) : stack
                        Just n -> Right n : stack
    put (ignore, new_stack)
    return new_stack
    
   