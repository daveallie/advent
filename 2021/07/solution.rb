# frozen_string_literal: true

class Day07
  def initialize(input)
    @crabs = input.split(',').map(&:to_i).sort
  end

  def part1
    median =
      if @crabs.length.even?
        (@crabs[@crabs.length / 2 - 1] + @crabs[@crabs.length / 2]) / 2
      else
        @crabs[@crabs.length / 2]
      end

    @crabs.map { |c| (median - c).abs }.sum
  end

  def part2
    min_crab, max_crab = @crabs.minmax
    (min_crab..max_crab).map { |pos| part2_subsolve(pos) }.min
  end

  private

  def part2_subsolve(pos)
    @crabs.map do |c|
      dist = (pos - c).abs
      dist * (dist + 1) / 2
    end.sum
  end
end

input = ($stdin.read || '').split("\n").first
Day07.new(input).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
