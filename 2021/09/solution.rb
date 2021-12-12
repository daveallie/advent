# frozen_string_literal: true

require 'set'

class Day09
  def initialize(lines)
    @grid = lines.map { |line| line.chars.map(&:to_i) }
  end

  def part1
    total = 0
    row_count = @grid.length
    col_count = @grid.first.length

    @grid.each_with_index do |row, row_num|
      row.each_with_index do |cell, col_num|
        min_adj = [
          (@grid[row_num - 1][col_num] if row_num.positive?),
          (@grid[row_num + 1][col_num] if row_num < row_count - 1),
          (@grid[row_num][col_num - 1] if col_num.positive?),
          (@grid[row_num][col_num + 1] if col_num < col_count - 1)
        ].compact.min

        total += cell + 1 if cell < min_adj
      end
    end

    total
  end

  def part2
    visited = Set.new
    sizes = []

    row_count = @grid.length
    col_count = @grid.first.length

    @grid.each_with_index do |row, start_r|
      row.each_with_index do |v, start_c|
        start_cell = [start_r, start_c]
        next if visited.include?(start_cell) || v == 9

        size = 0
        queue = [start_cell]

        loop do
          cell = queue.shift
          break unless cell

          r, c = cell

          next if visited.include?(cell) || @grid[r][c] == 9

          visited.add(cell)
          size += 1
          queue += [
            ([r - 1, c] if r.positive?),
            ([r + 1, c] if r < row_count - 1),
            ([r, c - 1] if c.positive?),
            ([r, c + 1] if c < col_count - 1)
          ].compact
        end

        sizes << size
      end
    end

    sizes.sort.last(3).reduce(:*)
  end
end

lines = ($stdin.read || '').split("\n")
Day09.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
