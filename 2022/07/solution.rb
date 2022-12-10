# frozen_string_literal: true

class Directory
  attr_reader :parent, :directories, :files

  def initialize(parent = nil)
    @parent = parent
    @files = {}
    @directories = {}
  end

  def dir_size
    @dir_size ||= @files.values.sum + @directories.values.sum(&:dir_size)
  end

  def add_file(name, size)
    @files[name] = size
  end

  def add_directory(name, parent = nil)
    @directories[name] = Directory.new(parent)
  end

  def each(&block)
    yield self
    @directories.each_value do |d|
      d.each(&block)
    end
  end
end

class Day07
  def initialize(lines)
    process_input(lines)
  end

  def process_input(lines)
    @root_dir = Directory.new
    curr_dir = @root_dir
    lines[1..].each do |line|
      next if line == '$ ls'

      if line == '$ cd ..'
        curr_dir = curr_dir.parent
        next
      end

      if line.start_with?('$ cd ')
        curr_dir = curr_dir.directories[line[5..]]
        next
      end

      size, name = line.split(' ', 2)
      if size == 'dir'
        curr_dir.add_directory(name, curr_dir)
      else
        curr_dir.add_file(name, size.to_i)
      end
    end
  end

  def part1
    total = 0
    @root_dir.each do |dir|
      total += dir.dir_size if dir.dir_size <= 100_000
    end
    total
  end

  def part2
    min = @root_dir.dir_size - 40_000_000
    target = Float::INFINITY

    @root_dir.each do |dir|
      target = dir.dir_size if dir.dir_size >= min && dir.dir_size < target
    end
    target
  end
end

lines = ($stdin.read || '').split("\n")
Day07.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end
