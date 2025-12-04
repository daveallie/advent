# frozen_string_literal: true

class Day03
  def initialize(lines)
    @lines = lines
  end

  def part1
    @lines.map do |line|
      dp(line.chars, 2)
    end.sum
  end

  def part2
    @lines.map do |line|
      dp(line.chars, 12)
    end.sum
  end

  private

  def dp(digs, max_length, solutions = Hash.new(-Float::INFINITY))
    return nil if max_length == 0
    return solutions[[digs, max_length]] if solutions.key?([digs, max_length])
    return digs.join.to_i if digs.length == max_length

    max = -1
    (0..digs.length - max_length - 1).each do |start_index|
      candidate = [
        (digs[start_index] + dp(digs[(start_index + 1)..], max_length - 1, solutions).to_s).to_i,
        dp(digs[(start_index + 1)..], max_length, solutions),
      ].max
      max = candidate if candidate > max
    end

    solutions[[digs, max_length]] = max
    max
  end
end

lines = ($stdin.read || '').split("\n")
Day03.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
