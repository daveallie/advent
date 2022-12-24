# frozen_string_literal: true

class Day14
  def initialize(lines)
    @input = lines[0]
    @pairs = @input.chars.each_cons(2).map { |a, b| a + b }.group_by(&:itself).transform_values(&:count)
    @insertions = lines[2..].to_h do |line|
      input, output = line.split(' -> ')
      [input, [input[0] + output, output + input[1]]]
    end
  end

  def part1
    solve(10)
  end

  def part2
    solve(40)
  end

  private

  def solve(steps)
    pair_count = @pairs

    steps.times do
      next_pair_count = Hash.new { |h, k| h[k] = 0 }
      pair_count.each do |pair, count|
        if @insertions.key?(pair)
          @insertions[pair].each do |insertion|
            next_pair_count[insertion] += count
          end
        else
          next_pair_count[pair] += count
        end
      end

      pair_count = next_pair_count
    end

    occurrences = Hash.new { |h, k| h[k] = 0 }
    pair_count.each do |pair, count|
      occurrences[pair[0]] += count
    end
    occurrences[@input[-1]] += 1

    min_occ, max_occ = occurrences.values.minmax
    max_occ - min_occ
  end
end

lines = ($stdin.read || '').split("\n")
Day14.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
