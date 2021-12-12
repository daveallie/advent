# frozen_string_literal: true

PART1_POINT_MAP = {
  ')' => 3,
  ']' => 57,
  '}' => 1197,
  '>' => 25137
}.freeze

PART2_POINT_MAP = {
  ')' => 1,
  ']' => 2,
  '}' => 3,
  '>' => 4
}.freeze

REV_MAP = {
  '(' => ')',
  '[' => ']',
  '{' => '}',
  '<' => '>'
}.freeze

class Day10
  def initialize(lines)
    @lines = lines
  end

  def part1
    @lines
      .map { |line| process(line) }
      .filter { |output| output[:type] == :illegal }
      .map { |output| PART1_POINT_MAP[output[:char]] }
      .sum
  end

  def part2
    points = @lines.map { |line| process(line) }.filter { |output| output[:type] == :incomplete }.map do |output|
      output[:stack].reverse.reduce(0) do |acc, char|
        (acc * 5) + PART2_POINT_MAP[char]
      end
    end

    points.sort[(points.length - 1) / 2]
  end

  private

  def process(line)
    stack = []

    line.chars.each do |char|
      if REV_MAP.key?(char)
        stack.push(REV_MAP[char])
      elsif stack.empty? || stack.pop != char
        return { type: :illegal, char: char }
      end
    end

    if stack.any?
      { type: :incomplete, stack: stack }
    else
      { type: :complete }
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day10.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
