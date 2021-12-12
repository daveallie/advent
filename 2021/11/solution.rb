# frozen_string_literal: true

require 'set'

class Day11
  def initialize(lines)
    @grid = lines.map { |line| line.chars.map(&:to_i) }
  end

  def part1
    grid = @grid.map(&:dup)
    total_flashes = 0

    100.times do
      total_flashes += flash(grid)
    end

    total_flashes
  end

  def part2
    grid = @grid.map(&:dup)
    iterations = 0

    loop do
      total_flashes = flash(grid)
      iterations += 1
      break if total_flashes == 100
    end

    iterations
  end

  private

  def flash(grid)
    flashed = Set.new
    queue_check = []
    row_count = grid.length
    col_count = grid[0].length

    grid.each_with_index do |row, r|
      row.each_with_index do |_, c|
        grid[r][c] += 1
        queue_check << [r, c] if grid[r][c] > 9
      end
    end

    loop do
      cell = queue_check.shift
      break if cell.nil?

      r, c = cell

      next if flashed.include?(cell)

      flashed.add(cell)

      new_cells = [
        ([r - 1, c] if r.positive?),
        ([r - 1, c + 1] if r.positive? && c < col_count - 1),
        ([r, c + 1] if c < col_count - 1),
        ([r + 1, c + 1] if r < row_count - 1 && c < col_count - 1),
        ([r + 1, c] if r < row_count - 1),
        ([r + 1, c - 1] if r < row_count - 1 && c.positive?),
        ([r, c - 1] if c.positive?),
        ([r - 1, c - 1] if r.positive? && c.positive?)
      ].compact

      new_cells.each do |new_cell|
        grid[new_cell[0]][new_cell[1]] += 1
        queue_check << new_cell if grid[new_cell[0]][new_cell[1]] > 9
      end
    end

    flashed.each do |cell|
      r, c = cell
      grid[r][c] = 0
    end

    flashed.length
  end
end

lines = ($stdin.read || '').split("\n")
Day11.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
