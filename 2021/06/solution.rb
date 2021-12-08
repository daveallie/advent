# frozen_string_literal: true

class Day06
  def initialize(input)
    @state = [0] * 9
    input.split(',').map(&:to_i).each { |i| @state[i] += 1 }
  end

  def part1
    run(80)
  end

  def part2
    run(256)
  end

  private

  def run(days)
    state = @state.dup

    days.times do
      bred_fish = state[0]
      state[0] = state[1]
      state[1] = state[2]
      state[2] = state[3]
      state[3] = state[4]
      state[4] = state[5]
      state[5] = state[6]
      state[6] = state[7] + bred_fish
      state[7] = state[8]
      state[8] = bred_fish
    end

    state.sum
  end
end

input = ($stdin.read || '').split("\n").first
Day06.new(input).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
