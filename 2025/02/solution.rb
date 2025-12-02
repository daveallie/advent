# frozen_string_literal: true

class Day02
  def initialize(ranges)
    @ranges = ranges
  end

  def part1
    solve(/^(.+)\1$/)
  end

  def part2
    solve(/^(.+)\1+$/)
  end

  private

  def solve(regex)
    sum = 0

    @ranges.each do |start, finish|
      (start..finish).each do |i|
        sum += i if i.to_s =~ regex
      end
    end

    sum
  end
end

ranges = ($stdin.read || '').split(",").map { |r| r.split('-').map(&:to_i) }
Day02.new(ranges).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
