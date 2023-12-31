# frozen_string_literal: true

class Day13
  def initialize(cases)
    @cases = cases
  end

  def part1
    solve(0)
  end

  def part2
    solve(1)
  end

  private

  def solve(differences)
    @cases.sum do |c|
      c_lines = c.split("\n")
      res = find_hline_sym(c_lines, differences)
      res ? res * 100 : find_vline_sym(c_lines, differences)
    end
  end

  def find_hline_sym(c_lines, differences)
    (1...c_lines.length).find { |i| assert_hline_sym(c_lines, i, differences) }
  end

  def find_vline_sym(c_lines, differences)
    find_hline_sym(c_lines.map(&:chars).transpose.map(&:join), differences)
  end

  # ind is the index of the line which the mirror is BEFORE
  def assert_hline_sym(c_lines, ind, differences)
    calc_diffs = c_lines[0...ind].each_with_index.sum do |l, i|
      matching_line = c_lines[ind * 2 - i - 1]
      if matching_line.nil?
        0
      else
        l.chars.zip(matching_line.chars).sum do |c1, c2|
          c1 == c2 ? 0 : 1
        end
      end
    end

    calc_diffs == differences
  end
end

cases = ($stdin.read || '').split("\n\n")
Day13.new(cases).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
