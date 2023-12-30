# frozen_string_literal: true

class Day11
  def initialize(lines)
    @data = lines.map(&:chars)
  end

  def part1
    # add 1 additional rows/cols (1 becomes 2)
    solve(1)
  end

  def part2
    # add 999,999 additional rows/cols (1 becomes 1,000,000)
    solve(999_999)
  end

  private

  def solve(scale)
    galaxies.combination(2).map do |(x1, y1), (x2, y2)|
      (x1 + extra_cols(x1) * scale - x2 - extra_cols(x2) * scale).abs +
        (y1 + extra_rows(y1) * scale - y2 - extra_rows(y2) * scale).abs
    end.sum
  end

  def galaxies
    return @galaxies if @galaxies

    @galaxies = []
    @data.each_with_index do |row, y|
      row.each_with_index do |cell, x|
        @galaxies << [x, y] if cell == '#'
      end
    end
    @galaxies
  end

  def extra_cols(x)
    return @extra_cols[x] if @extra_cols

    t_data = @data.transpose
    width = t_data.size
    @extra_cols = (0...width).map { |col| [col, 0] }.to_h

    empty_cols = t_data.each_index.find_all { |i| t_data[i].all?('.') }.reverse
    empty_cols.each do |col|
      (col...width).each do |future_x|
        @extra_cols[future_x] += 1
      end
    end

    @extra_cols[x]
  end

  def extra_rows(y)
    return @extra_rows[y] if @extra_rows

    height = @data.size
    @extra_rows = (0...height).map { |row| [row, 0] }.to_h

    empty_rows = @data.each_index.find_all { |i| @data[i].all?('.') }.reverse
    empty_rows.each do |row|
      (row...height).each do |future_y|
        @extra_rows[future_y] += 1
      end
    end

    @extra_rows[y]
  end
end

lines = ($stdin.read || '').split("\n")
Day11.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
