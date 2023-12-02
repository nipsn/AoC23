import re

# solution = 54676
# solution testinput2 = 281

# kinda janky but idc
translation = [
    (r"one", r"o1e"), (r"two", r"t2o"), (r"three", r"3e"), (r"four", r"4"), (r"five", r"5e"),
    (r"six", r"6"), (r"seven", r"7n"), (r"eight", r"8t"), (r"nine", r"9e")
]

# ultimate test
test = "oneightwoneight"
for m, s in translation:
    test = re.sub(m, s, test)
print(test)


with open("../input") as f:
    lines = f.readlines()
    curated = []
    for elem in lines:
        for m, s in translation:
            elem = re.sub(m, s, elem)
        curated.append(elem)
    print(sum([int(n[0] + n[-1]) for n in [re.sub(r"\D", "", l) for l in curated]]))


# import re
#
# input_file = open("input", "r")
# input = input_file.readlines()
# res = 0
# letters_dict = {'one':'1', 'two':'2', 'three':'3', 'four':'4', 'five':'5', 'six':'6', 'seven':'7', 'eight':'8', 'nine':'9'}
#
# for line in input:
#     index_start = 0
#     index_end = 0
#     for letters in letters_dict:
#         index_start = line.find(letters)
#         if index_start != -1:
#             line = line[:index_start+1] + letters_dict[letters] + line[index_start+1:]
#
#         index_end = line.rfind(letters)
#         if index_end != -1:
#             line = line[:index_end+1] + letters_dict[letters] + line[index_end+1:]
#
#     digits = re.findall(r"\d", line)
#     res_line = int(digits[0]+digits[-1])
#     res += res_line

# print(res)

