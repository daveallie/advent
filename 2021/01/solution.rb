# frozen_string_literal: true

class Day01
  def initialize(lines)
    @nums = lines.map(&:to_i)
  end

  def part1
    @nums.each_cons(2).count do |a, b|
      a < b
    end
  end

  def part2
    @nums.each_cons(4).count do |a, b, c, d|
      a + b + c < b + c + d
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day01.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
