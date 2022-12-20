# frozen_string_literal: true

Num = Struct.new(:value, :original_index, :next, :prev)

class Day20
  def initialize(lines)
    @lines = lines
  end

  def part1
    solve(build_nums)
  end

  def part2
    solve(build_nums(multiplier: 811_589_153), iters: 10)
  end

  private

  def build_nums(multiplier: 1)
    # build linked list
    nums = @lines.map.with_index { |line, index| Num.new(line.to_i * multiplier, index, nil, nil) }
    nums.each_cons(2) do |a, b|
      a.next = b
      b.prev = a
    end
    nums[0].prev = nums[-1]
    nums[-1].next = nums[0]
    nums
  end

  def solve(nums, iters: 1)
    iters.times do
      nums.each do |num|
        next if num.value.zero?

        # connect the two surrounding numbers
        num.prev.next = num.next
        num.next.prev = num.prev

        # get the new next value to insert before
        places = num.value % (nums.size - 1)
        new_next = num.next
        places.times do
          new_next = new_next.next
        end

        # insert the number
        new_prev = new_next.prev
        num.prev = new_prev
        num.next = new_next
        new_prev.next = num
        new_next.prev = num
      end
    end

    # sum 1000th, 2000th, 3000th number after 0
    curr = nums.find { |num| num.value.zero? }
    sum = 0
    3.times do
      (1000 % nums.size).times do
        curr = curr.next
      end
      sum += curr.value
    end
    sum
  end
end

lines = ($stdin.read || '').split("\n")
Day20.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
