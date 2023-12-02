import re
import math

# solution = 62811
# solution testinput = 2286

id_game = r"Game (\d+): (.*)"
red = r"[ ]*(\d+) red"
green = r"[ ]*(\d+) green"
blue = r"[ ]*(\d+) blue"


def sp(s: str, p: str) -> int:
    if match := re.search(p, s, re.IGNORECASE):
        return int(match.group(1))
    else:
        return 0


def min_color(l: list, i: int) -> tuple:
    return max(l, key=lambda x: x[i])[i]


with open("../input") as f:
    lines = f.readlines()
    games = {}
    for game in lines:
        if match := re.search(id_game, game, re.IGNORECASE):
            hands = [(sp(hand, red), sp(hand, green), sp(hand, blue)) for hand in game.split(";")]
            games[int(match.group(1))] = math.prod([min_color(hands, 0), min_color(hands, 1), min_color(hands, 2)])

    print(sum(games.values()))

