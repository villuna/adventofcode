import Data.Map.Strict (Map, lookup, insert, empty)
import Control.Monad.State.Lazy

numDigits :: Integer -> Int
numDigits 0 = 0
numDigits n = 1 + numDigits (div n 10)

split :: Integer -> Int -> [Integer]
split n d = [high, low]
    where
        high = n `div` (10^d)
        low = n `mod` (10^d)

update :: Integer -> [Integer]
update 0 = [1]
update n
    | even (numDigits n) = split n (numDigits n `div` 2)
    | otherwise = [n * 2024]

type Cache = State (Map (Integer, Integer) Integer)

expand :: Integer -> Integer -> Cache Integer
expand 0 _ = return 1
expand cycles n = do
    cache <- get

    case Data.Map.Strict.lookup (cycles, n) cache of
        Just r -> return r
        Nothing -> do
            result <- sum <$> mapM (expand (cycles - 1)) (update n)
            cache <- get
            put (insert (cycles, n) result cache)
            return result

main :: IO ()
main = do
    numbers <- map read . words <$> readFile "../input/day11.txt"
    let (part1, cache) = runState (sum <$> mapM (expand 25) numbers) empty

    putStr "Part 1: "
    print part1

    putStr "Part 2: "
    print $ evalState (sum <$> mapM (expand 75) numbers) cache