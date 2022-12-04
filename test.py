import re
regex = r"(Sent.*PM)"

test_str = open("noe","r")

matches = re.finditer(regex, test_str)

for matchNum, match in enumerate(matches, start=1):
    print ("Match {matchNum} was found at {start}-{end}: {match}".format(matchNum = matchNum, start = match.start(), end = match.end(), match = match.group()))

for groupNum in range(0, len(match.groups())):
    groupNum = groupNum + 1
    print ("Group {groupNum} found at {start}-{end}: {group}".format(groupNum = groupNum, start = match.start(groupNum), end = match.end(groupNum), group = match.group(groupNum)))