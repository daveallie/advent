# frozen_string_literal: true

class Day02
  def initialize(lines)
    @lines = lines
  end

  def part1
    @lines.map do |line|
      score(line.chars.first.ord - 'A'.ord + 1, line.chars.last.ord - 'X'.ord + 1)
    end.sum
  end

  def part2
    @lines.map do |line|
      rps_them = line.chars.first.ord - 'A'.ord + 1
      rps_me =
        case line.chars.last
        when 'X'
          loss(rps_them)
        when 'Y'
          rps_them
        else
          win(rps_them)
        end

      score(rps_them, rps_me)
    end.sum
  end

  private

  def loss(rps)
    case rps
    when 1
      3
    when 2
      1
    else
      2
    end
  end

  def win(rps)
    case rps
    when 3
      1
    when 1
      2
    else
      3
    end
  end

  def score(rps_them, rps_me)
    if win(rps_them) == rps_me
      6 + rps_me
    elsif loss(rps_them) == rps_me
      rps_me
    else
      # draw
      3 + rps_me
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day02.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
