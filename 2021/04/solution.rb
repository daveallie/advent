# frozen_string_literal: true

class Board
  def initialize(board_lines)
    @lines = board_lines.map { |line| line.strip.split(/\s+/).map(&:to_i) }
    @lines += @lines.transpose
  end

  def mark(num)
    @lines.each do |line|
      line.reject! { |cell| cell == num }
    end
  end

  def solved?
    @lines.any? { |line| line.empty? }
  end

  def solve_num
    @lines[0..4].map(&:sum).sum
  end
end

class Day04
  def initialize(lines)
    @input_nums = lines[0].split(',').map(&:to_i)
    @boards = lines[2..-1].each_slice(6).map { |board_lines| Board.new(board_lines[0..4]) }
  end

  def part1
    @input_nums.each do |num|
      @boards.each do |board|
        board.mark(num)
        return board.solve_num * num if board.solved?
      end
    end
  end

  def part2
    remaining_boards = @boards.dup

    @input_nums.each do |num|
      remove_boards = []
      remaining_boards.each do |board|
        board.mark(num)
        if board.solved?
          return board.solve_num * num if remaining_boards.length == 1

          remove_boards << board if board.solved?
        end
      end
      remaining_boards -= remove_boards
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day04.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
