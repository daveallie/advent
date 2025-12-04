# frozen_string_literal: true

class Day04
  def initialize(lines)
    @lines = lines
  end

  def part1
    setup
    remove_once
  end

  def part2
    setup
    sum = 0
    while true
      new_removals = remove_once
      break if new_removals == 0

      sum += new_removals
    end
    sum
  end

  private

  def setup
    @occupied = Set.new
    @neighbour_count = Hash.new(0)

    @lines.each_with_index do |line, row|
      line.chars.each_with_index do |char, col|
        next if char != '@'

        @occupied.add([row, col])
        (-1..1).each do |dr|
          (-1..1).each do |dc|
            next if dr == 0 && dc == 0

            @neighbour_count[[row + dr, col + dc]] += 1
          end
        end
      end
    end
  end

  def remove_once
    to_remove = @occupied.filter do |(row, col)|
      @neighbour_count[[row, col]] < 4
    end

    to_remove.each do |(row, col)|
      @occupied.delete([row, col])

      (-1..1).each do |dr|
        (-1..1).each do |dc|
          next if dr == 0 && dc == 0

          @neighbour_count[[row + dr, col + dc]] -= 1
        end
      end
    end

    to_remove.size
  end
end

lines = ($stdin.read || '').split("\n")
Day04.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
