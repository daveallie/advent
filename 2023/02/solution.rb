# frozen_string_literal: true

class Game
  attr_reader :id, :sets

  def initialize(line)
    game_id, data = line.split(': ')

    @id = game_id.split(' ')[1].to_i
    @maxes = { 'red' => 0, 'green' => 0, 'blue' => 0 }

    data.split('; ').each do |set|
      set.split(', ').each do |draw|
        count, color = draw.split(' ')
        count = count.to_i
        @maxes[color] = count if count > @maxes[color]
      end
    end
  end

  def possible?
    @maxes['red'] <= 12 && @maxes['green'] <= 13 && @maxes['blue'] <= 14
  end

  def power
    @maxes.values.reduce(:*)
  end
end

class Day02
  def initialize(lines)
    @games = lines.map { |line| Game.new(line) }
  end

  def part1
    @games.filter(&:possible?).sum(&:id)
  end

  def part2
    @games.sum(&:power)
  end
end

lines = ($stdin.read || '').split("\n")
Day02.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
