import re

# solution input = 53921
# solution testinput1 = 142
with open("../input") as f:
    print(sum([int(n[0] + n[-1]) for n in [re.sub(r"\D", "", l) for l in f.readlines()]]))
