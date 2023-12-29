# frozen_string_literal: true

# this is just a quadratic equation
#   distance_travelled = time_held * (race_time - time_held)
#
# we want to solve the equation when
#   distance_travelled is the current race record + 1 (called win_distance)
#   race_time is the race time limit
#   time_held is the variable we're solving for
#
# so
#   win_distance = time_held * (race_time - time_held)
#   0 = time_held^2 - race_time * time_held + win_distance
#
# quadratic formula:
#   time_held = (race_time +- sqrt(race_time^2 - 4 * win_distance)) / 2
#
# solutions will be fractional
# so we round up for the minimum, and round down for the maximum

class Day06
  def initialize(lines)
    @lines = lines
  end

  def part1
    times = @lines[0].split(':').last.split(' ').map(&:to_i)
    distances = @lines[1].split(':').last.split(' ').map(&:to_i)

    times.zip(distances).map do |time, distance|
      solve(time, distance + 1)
    end.inject(:*)
  end

  def part2
    time = @lines[0].split(':').last.tr(' ', '').to_i
    distance = @lines[1].split(':').last.tr(' ', '').to_i

    solve(time, distance)
  end

  private

  def solve(time, win_distance)
    discriminant = time.pow(2) - 4 * win_distance
    raise 'unwinnable' if discriminant < 0

    sqrt_discriminant = Math.sqrt(discriminant)
    min_hold = ((time - sqrt_discriminant) / 2).ceil
    max_hold = ((time + sqrt_discriminant) / 2).floor

    max_hold - min_hold + 1
  end
end

lines = ($stdin.read || '').split("\n")
Day06.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
