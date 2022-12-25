# frozen_string_literal: true

require 'set'

Point = Struct.new(:x, :y) do
  def adjacents
    [
      Point.new(x, y - 1),
      Point.new(x, y + 1),
      Point.new(x - 1, y),
      Point.new(x + 1, y)
    ]
  end
end

class Day15
  def initialize(lines)
    @size = lines.size
    @weights = lines.each_with_index.flat_map do |line, y|
      line.chars.each_with_index.flat_map do |char, x|
        5.times.flat_map do |i|
          5.times.map do |j|
            val = char.to_i
            [Point.new(x + i * @size, y + j * @size), ((val + i + j - 1) % 9) + 1]
          end
        end
      end
    end.to_h
  end

  def part1
    dijk(Point.new(0, 0), Point.new(@size - 1, @size - 1))
  end

  def part2
    dijk(Point.new(0, 0), Point.new(@size * 5 - 1, @size * 5 - 1), part2: true)
  end

  private

  def dijk(start, finish, part2: false)
    dist = Hash.new(Float::INFINITY)
    dist[start] = 0
    seen = Set.new
    queue = [start]

    until queue.empty?
      point = queue.min_by { |p| dist[p] }
      queue.delete(point)
      seen.add(point)

      break if point == finish

      point.adjacents.each do |neighbour|
        next unless @weights.key?(neighbour)
        next if !part2 && (neighbour.x >= @size || neighbour.y >= @size)

        queue << neighbour unless seen.include?(neighbour)
        dist[neighbour] = [dist[neighbour], dist[point] + @weights[neighbour]].min
      end
    end

    dist[finish]
  end
end

lines = ($stdin.read || '').split("\n")
Day15.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
