# frozen_string_literal: true

require 'set'

class Day08
  def initialize(lines)
    @heights = lines.map { |line| line.chars.map(&:to_i) }
  end

  def part1
    # track the visible height of each cell from each direction
    visible_height_top = Array.new(@heights.size) { @heights.first.dup }
    visible_height_bottom = Array.new(@heights.size) { @heights.last.dup }
    visible_height_left = Array.new(@heights.first.size) { @heights.map(&:first) }.transpose
    visible_height_right = Array.new(@heights.first.size) { @heights.map(&:last) }.transpose
    visible = Set.new

    (@heights.size - 2).times do |row_idx|
      (@heights.first.size - 2).times do |col_idx|
        tl_row = row_idx + 1
        tl_col = col_idx + 1
        br_row = @heights.size - row_idx - 2
        br_col = @heights.first.size - col_idx - 2

        visible_height_top[tl_row][tl_col] = [visible_height_top[tl_row - 1][tl_col], @heights[tl_row][tl_col]].max
        visible_height_left[tl_row][tl_col] = [visible_height_left[tl_row][tl_col - 1], @heights[tl_row][tl_col]].max
        if @heights[tl_row][tl_col] > visible_height_top[tl_row - 1][tl_col] ||
           @heights[tl_row][tl_col] > visible_height_left[tl_row][tl_col - 1]
          visible.add([tl_row, tl_col])
        end

        visible_height_bottom[br_row][br_col] =
          [visible_height_bottom[br_row + 1][br_col], @heights[br_row][br_col]].max
        visible_height_right[br_row][br_col] = [visible_height_right[br_row][br_col + 1], @heights[br_row][br_col]].max
        if @heights[br_row][br_col] > visible_height_bottom[br_row + 1][br_col] ||
           @heights[br_row][br_col] > visible_height_right[br_row][br_col + 1]
          visible.add([br_row, br_col])
        end
      end
    end

    @heights.size * 2 + (@heights.first.size - 2) * 2 + visible.size
  end

  def part2
    @heights.map.with_index do |row, row_idx|
      row.map.with_index do |height, col_idx|
        num_visible(row_idx, col_idx, height, -1, 0) *
          num_visible(row_idx, col_idx, height, 1, 0) *
          num_visible(row_idx, col_idx, height, 0, -1) *
          num_visible(row_idx, col_idx, height, 0, 1)
      end
    end.flatten.max
  end

  private

  def num_visible(row, col, height, row_diff, col_diff)
    return 0 if row.zero? || row == @heights.size - 1 || col.zero? || col == @heights.first.size - 1
    return 1 if @heights[row + row_diff][col + col_diff] >= height

    1 + num_visible(row + row_diff, col + col_diff, height, row_diff, col_diff)
  end
end

lines = ($stdin.read || '').split("\n")
Day08.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
