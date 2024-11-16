import requests

ADVENT_OF_CODE_URL = 'https://www.adventofcode.com/'
ADVENT_OF_CODE_INPUT_URL = f'{ADVENT_OF_CODE_URL}/{{year}}/day/{{day}}/input'


def read_session():
    with open('./secrets/session.txt', 'r') as file:
        return file.read().strip()


def get(year, day):
    return requests.get(
        ADVENT_OF_CODE_INPUT_URL.format(day=day, year=year),
        cookies={
            'User-Agent': 'advent-of-code-input-fetcher',
            'session': read_session()
        }
    )


year = int(input('enter year to fetch input for: '))
day = int(input('enter day to fetch input for: '))
data = get(year, day)
data.raise_for_status()

with open(f'./{year}-{day}.txt', 'w') as file:
    file.write(data.text)
