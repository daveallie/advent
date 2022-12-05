# frozen_string_literal: true

class Day05
  def initialize(lines)
    raw_stacks, @moves = lines.slice_after('').to_a
    @stacks = raw_stacks[...-2]
                .map { |line| line.chars.each_slice(4).map { |c| c[1] } }
                .transpose
                .map { |stack| stack.filter { |crate| crate != ' ' }.reverse }
  end

  def part1
    stacks = @stacks.map(&:dup)
    @moves.each do |move|
      _, count, _, from, _, to = move.split(' ')
      count.to_i.times do
        stacks[to.to_i - 1] << stacks[from.to_i - 1].pop
      end
    end

    stacks.map(&:last).join
  end

  def part2
    stacks = @stacks.map(&:dup)
    @moves.each do |move|
      _, count, _, from, _, to = move.split(' ')

      stacks[to.to_i - 1] += stacks[from.to_i - 1].pop(count.to_i)
    end

    stacks.map(&:last).join
  end
end

lines = ($stdin.read || '').split("\n")
Day05.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
