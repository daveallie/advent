solve1 :: [Int] -> Int
solve1 nums = head [a * b | a <- nums, b <- nums, a + b == 2020]

solve2 :: [Int] -> Int
solve2 nums = head [a * b * c | a <- nums, b <- nums, c <- nums, a + b + c == 2020]

main :: IO ()
main = do
  inputs <- fmap lines (readFile "input")
  let nums = map read inputs :: [Int]
  print (solve2 nums)

