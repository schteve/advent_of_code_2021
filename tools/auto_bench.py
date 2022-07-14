import re
import subprocess

unit_factors = {
    's': 1,
    'ms': 1000,
    'us': 1000000,
    'ns': 1000000000,
}

time_regex = re.compile(r'time:\s*\[\d+(?:\.\d+)?\s\w+ (\d+(?:\.\d+))\s(\w+) \d+(?:\.\d+)?\s\w+\]')

# Read README.md so we can insert the benchmark results as we go
with open('README.md', 'r') as readme:
    readme_text = readme.read()

total = 0
for day in range(1, 26):
    for part in range(1, 3):
        if day == 25 and part == 2:
            continue # No such thing as day 25 part 2

        # Run the bench command
        result = subprocess.run(['cargo', 'aoc', 'bench', '-d', f'{day}', '-p',f'{part}'], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        result_output = result.stdout.decode('utf-8')

        # Extract the median time
        (val, unit) = time_regex.search(result_output).groups()
        print(f'Day {day} part {part}: {val} {unit}')

        # Calculate the adjusted time (units of seconds) and track the total as we go
        time_adjusted = float(val) / unit_factors[unit]
        total += time_adjusted

        # Insert the time into the readme
        readme_text = re.sub(rf'^({day}\s+\| {part}\s+\| )(.*?)$', rf'\g<1>{val} {unit}', readme_text, flags=re.MULTILINE)

# Insert the total time into the readme
print(f'Total time: {total:.5} s')
readme_text = re.sub(r'^Total: .*?$', rf'Total: {total:.5} s', readme_text, flags=re.MULTILINE)

with open('README.md', 'w') as readme:
    readme.write(readme_text)
