# frozen_string_literal: true

class Day06
  def initialize(line)
    @line = line
  end

  def part1
    solve(4)
  end

  def part2
    solve(14)
  end

  def solve(length)
    length + @line.chars.each_cons(length).find_index do |chars|
      chars.uniq.size == length
    end
  end
end

line = ($stdin.read || '').split("\n")[0]
Day06.new(line).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
