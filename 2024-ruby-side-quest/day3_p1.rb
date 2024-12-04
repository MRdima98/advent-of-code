re = /mul\(\d+,\d+\)/
re_num = /\d+/
nums = []
couples = []

File.readlines('input', chomp: true).each do |line|
    match = line.scan re
    match.each do |m|
        nums = m.scan re_num
        couples << nums[0].to_i * nums[1].to_i
    end
end

puts couples.inject(:+)

