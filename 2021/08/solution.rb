# frozen_string_literal: true

SEGMENTS = [
  'abcefg',  # 0
  'cf',      # 1
  'acdeg',   # 2
  'acdfg',   # 3
  'bcdf',    # 4
  'abdfg',   # 5
  'abdefg',  # 6
  'acf',     # 7
  'abcdefg', # 8
  'abcdfg'   # 9
].freeze

class Day08
  def initialize(lines)
    @lines = lines
  end

  def part1
    @lines.map { |line| line.split(' | ')[1].split(' ').count { |dig| [2, 3, 4, 7].include?(dig.length) } }.sum
  end

  def part2
    @lines.map do |line|
      signals, solution = line.split(' | ').map { |part| part.split(' ').map(&:chars) }

      one = signals.find { |signal| signal.length == 2 }
      seven = signals.find { |signal| signal.length == 3 }
      two_three_five = signals.find_all { |signal| signal.length == 5 }
      zero_six_nine = signals.find_all { |signal| signal.length == 6 }

      cf_chars = one
      a_char = (seven - cf_chars).first

      dg_chars = two_three_five.reduce(:&) - [a_char]
      abfg_chars = zero_six_nine.reduce(:&)

      g_char = (abfg_chars & dg_chars).first
      d_char = (dg_chars - [g_char]).first

      bf_chars = abfg_chars - [a_char, g_char]

      f_char = (one & bf_chars).first
      b_char = (bf_chars - [f_char]).first
      c_char = (cf_chars - [f_char]).first
      e_char = (%w[a b c d e f g] - [a_char, b_char, c_char, d_char, f_char, g_char]).first

      mapping = {
        a_char => 'a',
        b_char => 'b',
        c_char => 'c',
        d_char => 'd',
        e_char => 'e',
        f_char => 'f',
        g_char => 'g'
      }

      solution.map do |crossed_num|
        uncrossed_num = crossed_num.map { |char| mapping[char] }.sort.join
        SEGMENTS.index(uncrossed_num)
      end.join.to_i
    end.sum
  end
end

lines = ($stdin.read || '').split("\n")
Day08.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
