# Calculate the total time to run all of the days and parts.
# Also print out some of the longest run times.

import re

unit_factors = {
    's': 1,
    'ms': 1000,
    'us': 1000000,
    'ns': 1000000000,
}

db = {}
with open('README.md') as fin:
    regex = re.compile('''(\d+)\s+\|\s+(\d+)\s+\|\s+(\d+\.?\d+)\s(\w+)''')
    for line in fin:
        result = regex.match(line)
        if result:
            day = result.group(1)
            part = result.group(2)
            time = result.group(3)
            unit = result.group(4)

            time_adjusted = float(time) / unit_factors[unit]
            db[(day, part)] = time_adjusted

# Turn the db into a list and sort by time (descending)
db_list = sorted(db.items(), key=lambda x: x[1], reverse=True)

print('Total time: {:.5} ({} parts)'.format(sum(db.values()), len(db)))
print('Biggest 5:')
for i in range(5):
    print('    Day {}, part {}: {:.5} s'.format(db_list[i][0][0], db_list[i][0][1], db_list[i][1]))
