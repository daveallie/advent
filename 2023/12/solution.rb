# frozen_string_literal: true

class Day12
  def initialize(lines)
    @cases = lines.map do |line|
      data, arrangements = line.split(' ')
      { data: data, arrangements: arrangements.split(',').map(&:to_i) }
    end
  end

  def part1
    solve(1)
  end

  def part2
    solve(5)
  end

  private

  def solve(repeat_count)
    @memo ||= {}
    @cases.sum do |c|
      c = simplify({
                     data: ([c[:data]] * repeat_count).join('?'),
                     arrangements: c[:arrangements] * repeat_count
                   })

      count_options(c)
    end
  end

  def count_options(c)
    # memo exists
    return @memo[c] if @memo.key?(c)

    # Base case, empty string, 1 solution only if no more arrangements
    return @memo[c] = c[:arrangements].empty? ? 1 : 0 if c[:data] == ''
    # Base case, no more arrangements, 1 solution only if all dots or wildcards
    return @memo[c] = c[:data].match?(/^[.?]+$/) ? 1 : 0 if c[:arrangements].empty?
    # Base case, string too short to fit all arrangements
    return @memo[c] = 0 if c[:data].length < (c[:arrangements].sum + c[:arrangements].length - 1)

    next_arr = c[:arrangements].first
    @memo[c] = 0

    # Try to solve with string starting with next arrangement
    if c[:data].match?(/^[#?]{#{next_arr}}([?.]|$)/)
      next_c = simplify({ data: c[:data][next_arr + 1..] || '', arrangements: c[:arrangements][1..] })
      @memo[c] += count_options(next_c)
    end

    # Try to solve with string starting with blank (.)
    if c[:data].start_with?('?')
      next_c = simplify({ data: c[:data][1..] || '', arrangements: c[:arrangements] })
      @memo[c] += count_options(next_c)
    end

    @memo[c]
  end

  def simplify(c)
    res = { data: c[:data].dup, arrangements: c[:arrangements].dup }

    # Remove leading and trailing dots
    res[:data].sub!(/^\.*/, '').sub!(/\.*$/, '') if res[:data].start_with?('.') || res[:data].end_with?('.')

    # Remove complete leading arrangements
    while res[:arrangements].any? && res[:data].match?(/^##{res[:arrangements].first}\./)
      res[:data].sub!(/^#+\.+/, '')
      res[:arrangements].shift
    end

    # Remove complete trailing arrangements
    while res[:arrangements].any? && res[:data].match?(/\.##{res[:arrangements].last}$/)
      res[:data].sub!(/\.#+$/, '')
      res[:arrangements].pop
    end

    res
  end
end

lines = ($stdin.read || '').split("\n")
Day12.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
