# frozen_string_literal: true

class Day09
  def initialize(lines)
    @sequences = lines.map { |line| line.split(' ').map(&:to_i) }
  end

  def part1
    @sequences.map { |s| get_next(s) }.sum
  end

  def part2
    @sequences.map { |s| get_prev(s) }.sum
  end

  private

  def get_next(sequence)
    return 0 if sequence.all?(&:zero?)

    sequence[-1] + get_next(sequence.each_cons(2).map { |a, b| b - a })
  end

  def get_prev(sequence)
    return 0 if sequence.all?(&:zero?)

    sequence[0] - get_prev(sequence.each_cons(2).map { |a, b| b - a })
  end
end

lines = ($stdin.read || '').split("\n")
Day09.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
