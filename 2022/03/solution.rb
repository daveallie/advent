# frozen_string_literal: true

class Day03
  def initialize(lines)
    @lines = lines
  end

  def part1
    @lines.map do |line|
      solve(line.chars.each_slice(line.length / 2).to_a)
    end.sum
  end

  def part2
    @lines.each_slice(3).map do |group|
      solve(group.map(&:chars))
    end.sum
  end

  private

  def solve(parts)
    char_val(parts.reduce(:&).first)
  end

  def char_val(char)
    val = char.ord
    val >= 'a'.ord ? val - 'a'.ord + 1 : val - 'A'.ord + 27
  end
end

lines = ($stdin.read || '').split("\n")
Day03.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
