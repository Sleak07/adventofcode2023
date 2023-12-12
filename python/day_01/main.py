
total = 0

with open("input.txt",'r') as f :
    for line in f.readlines():
        nums = [char for char in line .strip() if char.isdigit()]
        data = int(nums[0] + nums[-1])

        total += data
        print(total)
