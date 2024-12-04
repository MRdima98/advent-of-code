re = /do\(\)|don\'t\(\)|mul\(\d+,\d+\)/
re_num = /\d+/
nums = []
couples = []
mul = true

File.readlines('input', chomp: true).each do |line|
    match = line.scan re
    match.each do |m|
        # puts "This is m:", m
        # puts "This is mul:", mul
        if m.include? "do()" 
            puts "do"
                mul = true
        end

        if m.include? "don't()" 
            puts "dont"
                mul = false
        end

        nums = m.scan re_num
        molt = nums[0].to_i * nums[1].to_i
        couples << molt if mul and molt != 0
    end
end

puts couples.inject(:+)

