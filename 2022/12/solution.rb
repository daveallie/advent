# frozen_string_literal: true

require 'set'

class Day12
  def initialize(lines)
    @grid = lines.map.with_index do |line, row_idx|
      line.chars.map.with_index do |char, col_idx|
        case char
        when 'S'
          @start = [row_idx, col_idx]
          0
        when 'E'
          @finish = [row_idx, col_idx]
          25
        else
          char.ord - 'a'.ord
        end
      end
    end
  end

  def part1
    distance_to_finish(@start)
  end

  def part2
    @grid.map.with_index do |row, row_idx|
      row.map.with_index do |height, col_idx|
        height.zero? ? distance_to_finish([row_idx, col_idx]) : Float::INFINITY
      end
    end.flatten.min
  end

  private

  def distance_to_finish(start_node)
    return @distances_to_finish[start_node] if @distances_to_finish

    # Dijkstra's, but we traverse the path backwards to find the shortest path from the finish to all nodes
    @distances_to_finish = Hash.new(Float::INFINITY)
    @distances_to_finish[@finish] = 0
    seen = Set.new
    queue = [@finish]

    until queue.empty?
      node = queue.min_by { |un_node| @distances_to_finish[un_node] }
      queue.delete(node)
      seen.add(node)

      neighbours(node, seen).each do |neighbour|
        queue << neighbour
        @distances_to_finish[neighbour] = [@distances_to_finish[neighbour], @distances_to_finish[node] + 1].min
      end
    end

    @distances_to_finish[start_node]
  end

  def neighbours(node, seen)
    row_idx, col_idx = node
    [
      [row_idx - 1, col_idx],
      [row_idx + 1, col_idx],
      [row_idx, col_idx - 1],
      [row_idx, col_idx + 1]
    ].filter do |new_row, new_col|
      new_row >= 0 && new_row < @grid.size && new_col >= 0 && new_col < @grid.first.size &&
        # we're going backwards, so our current node must be lower (or at most 1 higher) than our new node
        @grid[row_idx][col_idx] - 1 <= @grid[new_row][new_col] &&
        !seen.include?([new_row, new_col])
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day12.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
