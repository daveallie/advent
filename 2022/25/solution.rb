# frozen_string_literal: true

class Day25
  def initialize(lines)
    @nums = lines.map do |line|
      from_snafu(line)
    end
  end

  def part1
    to_snafu(@nums.sum)
  end

  def part2
    # no part 2 :)
  end

  private

  def from_snafu(input)
    result = input.tr('=\-', '00').to_i(5)
    input.reverse.chars.each_with_index do |char, index|
      result -= (5**index) * 2 if char == '='
      result -= 5**index if char == '-'
    end
    result
  end

  def to_snafu(input)
    return '' if input.zero?

    char, carry =
      case input % 5
      when 0, 1, 2
        [(input % 5).to_s, 0]
      when 3
        ['=', 1]
      when 4
        ['-', 1]
      end

    to_snafu(input / 5 + carry) + char
  end
end

lines = ($stdin.read || '').split("\n")
Day25.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
