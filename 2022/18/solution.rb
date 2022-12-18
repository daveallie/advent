# frozen_string_literal: true

require 'set'

Point = Struct.new(:x, :y, :z) do
  def adjacent_forward
    [
      Point.new(x + 1, y, z),
      Point.new(x, y + 1, z),
      Point.new(x, y, z + 1)
    ]
  end

  def adjacent
    [
      Point.new(x - 1, y, z),
      Point.new(x, y - 1, z),
      Point.new(x, y, z - 1),
      *adjacent_forward
    ]
  end
end

class Day18
  def initialize(lines)
    @points = Set.new(lines.map { |line| Point.new(*line.split(',').map(&:to_i)) })
  end

  def part1
    total_faces = @points.size * 6
    @points.each do |point|
      point.adjacent_forward.each do |adjacent|
        total_faces -= 2 if @points.include?(adjacent)
      end
    end

    total_faces
  end

  def part2
    # bounding box of all the points
    min_x, max_x = @points.map(&:x).minmax
    min_y, max_y = @points.map(&:y).minmax
    min_z, max_z = @points.map(&:z).minmax
    min_x -= 1
    max_x += 1
    min_y -= 1
    max_y += 1
    min_z -= 1
    max_z += 1

    # bfs from a corner of the bounding box, counting faces
    queue = [Point.new(min_x, min_y, min_z)]
    visited = Set.new
    total_faces = 0
    while (point = queue.shift)
      next if visited.include?(point)

      visited << point
      point.adjacent.each do |adjacent|
        next if visited.include?(adjacent)
        next if adjacent.x < min_x || adjacent.x > max_x
        next if adjacent.y < min_y || adjacent.y > max_y
        next if adjacent.z < min_z || adjacent.z > max_z

        if @points.include?(adjacent)
          total_faces += 1
        else
          queue << adjacent
        end
      end
    end

    total_faces
  end
end

lines = ($stdin.read || '').split("\n")
Day18.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
