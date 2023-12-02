import re

# solution = 2551
# solution testinput = 8

id_game = r"Game (\d+): (.*)"
red = r"[ ]*(\d+) red"
green = r"[ ]*(\d+) green"
blue = r"[ ]*(\d+) blue"


def sp(s: str, p: str) -> int:
    if match := re.search(p, s, re.IGNORECASE):
        return int(match.group(1))
    else:
        return 0


def max_within_range(l: list, i: int) -> tuple:
    i_to_max = {0: 12, 1: 13, 2: 14}
    return i_to_max[i] >= max(l, key=lambda x: x[i])[i]


with open("../input") as f:
    lines = f.readlines()
    games = {}
    for game in lines:
        if match := re.search(id_game, game, re.IGNORECASE):
            hands = [(sp(hand, red), sp(hand, green), sp(hand, blue)) for hand in game.split(";")]
            games[int(match.group(1))] = (max_within_range(hands, 0), max_within_range(hands, 1), max_within_range(hands, 2))

    print(sum(dict(filter(lambda x: all(x[1]) == True, games.items())).keys()))

