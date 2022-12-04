# frozen_string_literal: true

class Day04
  def initialize(lines)
    @lines = lines.map do |line|
      line.split(',').map do |range|
        first, last = range.split('-').map(&:to_i)
        first..last
      end
    end
  end

  def part1
    @lines.count do |range1, range2|
      range1.cover?(range2) || range2.cover?(range1)
    end
  end

  def part2
    @lines.count do |range1, range2|
      range1.cover?(range2.first) || range2.cover?(range1.first)
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day04.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
