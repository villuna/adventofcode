import Data.List

parse = foldr ((\[x, y] [r1, r2] -> [x:r1, y:r2]) . map read . words) [[], []] . lines

absdiff x y = abs (x - y)
count x = length . filter (==x)

main = do
    [l1, l2] <- map sort . parse <$> readFile "../input/day1.txt"
    print $ sum (zipWith absdiff l1 l2)
    print $ sum (map (\x -> x * count x l2) l1)