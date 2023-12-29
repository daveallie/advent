# frozen_string_literal: true

CARD_VAL_MAP = {
  'A' => 14,
  'K' => 13,
  'Q' => 12,
  'J' => 11,
  'T' => 10,
  '9' => 9,
  '8' => 8,
  '7' => 7,
  '6' => 6,
  '5' => 5,
  '4' => 4,
  '3' => 3,
  '2' => 2,
  '*' => 1 # joker

}.freeze

class Hand
  attr_reader :cards

  def initialize(cards, jokers: false)
    cards = cards.tr('J', '*') if jokers

    @cards = cards.split('').map { |c| CARD_VAL_MAP[c] }
  end

  def hand_type
    return @hand_type if @hand_type

    counts = cards.each_with_object(Hash.new(0)) { |card, hash| hash[card] += 1 }
    num_jokers = counts[CARD_VAL_MAP['*']]
    counts.delete(CARD_VAL_MAP['*'])
    max_same = counts.values.max || 0

    # five of a kind
    return @hand_type = 7 if (max_same + num_jokers) == 5
    # four of a kind
    return @hand_type = 6 if (max_same + num_jokers) == 4
    # full house
    if max_same == 3 && counts.values.any?(2) ||
       # one joker
       counts.values.count(2) == 2 && num_jokers >= 1 ||
       max_same == 3 && num_jokers >= 1 ||
       # two jokers
       max_same == 2 && num_jokers >= 2
      return @hand_type = 5
    end
    # three of a kind
    return @hand_type = 4 if (max_same + num_jokers) == 3
    # two pair
    if counts.values.count(2) == 2 ||
       max_same == 2 && num_jokers >= 1 ||
       num_jokers >= 2
      return @hand_type = 3
    end
    # one pair
    return @hand_type = 2 if (max_same + num_jokers) == 2

    # high card
    @hand_type = 1
  end

  def <=>(other)
    return hand_type <=> other.hand_type if hand_type != other.hand_type

    cards.zip(other.cards).each do |card, other_card|
      return card <=> other_card if card != other_card
    end

    0
  end
end

class Day07
  def initialize(lines)
    @data = lines.map do |line|
      line.split(' ')
    end
  end

  def part1
    solve(false)
  end

  def part2
    solve(true)
  end

  private

  def solve(part2)
    @data.map { |cards, bid| { hand: Hand.new(cards, jokers: part2), bid: bid.to_i } }
         .sort_by { |a| a[:hand] }
         .each_with_index.map { |hand, index| hand[:bid] * (index + 1) }
         .sum
  end
end

lines = ($stdin.read || '').split("\n")
Day07.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
