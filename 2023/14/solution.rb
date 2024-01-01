# frozen_string_literal: true

class Day14
  def initialize(input)
    @grid = input.split("\n").map(&:chars)
  end

  def part1
    count_weight(move_north(@grid))
  end

  def part2
    grid = @grid
    cycle = 0
    max_cycles = 1000000000
    seen = [grid]

    loop do
      break if cycle >= max_cycles

      grid = move_east(move_south(move_west(move_north(grid))))
      cycle += 1

      break if seen.include?(grid)

      seen << grid
    end

    cycle_start = seen.index(grid)
    cycle_length = cycle - cycle_start

    ind = (max_cycles - cycle_start) % cycle_length + cycle_start
    count_weight(seen[ind])
  end

  private

  def move_north(grid)
    grid.transpose.map { |c| move_col_up(c) }.transpose
  end

  def move_east(grid)
    grid.map { |c| move_col_up(c.reverse).reverse }
  end

  def move_south(grid)
    grid.transpose.map { |c| move_col_up(c.reverse).reverse }.transpose
  end

  def move_west(grid)
    grid.map { |c| move_col_up(c) }
  end

  def move_col_up(col)
    return col if col.empty? || col.all?(/[#.]/)
    return ['#'] + move_col_up(col[1..]) if col[0] == '#'

    ind = col.index('#')
    count = col[..(ind || -1)].count('O')
    res = (['O'] * count) + (['.'] * ((ind || col.length) - count))
    res += ['#'] + move_col_up(col[ind + 1..]) if ind

    res
  end

  def count_weight(grid)
    grid.transpose.sum do |col|
      col.reverse.each_with_index.sum do |c, i|
        c == 'O' ? i + 1 : 0
      end
    end
  end
end

input = ($stdin.read || '')
Day14.new(input).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
