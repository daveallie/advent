# frozen_string_literal: true

NUMBER_MAP = {
  '1' => 1,
  '2' => 2,
  '3' => 3,
  '4' => 4,
  '5' => 5,
  '6' => 6,
  '7' => 7,
  '8' => 8,
  '9' => 9,
  'one' => 1,
  'two' => 2,
  'three' => 3,
  'four' => 4,
  'five' => 5,
  'six' => 6,
  'seven' => 7,
  'eight' => 8,
  'nine' => 9
}.freeze

class Day01
  def initialize(lines)
    @lines = lines
  end

  def part1
    solve(/\d/)
  end

  def part2
    solve(/(?=(one|two|three|four|five|six|seven|eight|nine|zero|\d))/)
  end

  private

  def solve(matcher)
    @lines.sum do |line|
      line_nums = line.scan(matcher).flatten
      NUMBER_MAP[line_nums[0]] * 10 + NUMBER_MAP[line_nums[-1]]
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day01.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
