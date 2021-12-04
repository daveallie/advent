# frozen_string_literal: true

class Day03
  def initialize(lines)
    @lines = lines
  end

  def part1
    most_common, least_common = most_and_least_common
    most_common.join.to_i(2) * least_common.join.to_i(2)
  end

  def part2
    index = 0
    oxy_rating_nums = @lines
    while oxy_rating_nums.size > 1
      ones, zeros = oxy_rating_nums.partition { |num| num[index] == '1' }
      oxy_rating_nums =
        if ones.length >= zeros.length
          ones
        else
          zeros
        end
      index += 1
    end

    index = 0
    co2_rating_nums = @lines
    while co2_rating_nums.size > 1
      ones, zeros = co2_rating_nums.partition { |num| num[index] == '1' }
      co2_rating_nums =
        if ones.length >= zeros.length
          zeros
        else
          ones
        end
      index += 1
    end


    oxy_rating_nums.first.to_i(2) * co2_rating_nums.first.to_i(2)
  end

  private

  def most_and_least_common
    size = @lines[0].length
    count = [0] * size

    @lines.each do |line|
      line.each_char.with_index do |char, x|
        if char == '1'
          count[x] += 1
        else
          count[x] -= 1
        end
      end
    end

    most_common = count.map do |c|
      if c.negative?
        0
      else
        1
      end
    end

    least_common = most_common.map { |i| 1 - i }

    [most_common, least_common]
  end
end

lines = ($stdin.read || '').split("\n")
Day03.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
