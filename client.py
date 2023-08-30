#!/usr/bin/env python3
"""Super simple script to just hammer the api with requests.
For now i just wanted to throw all kinds of valid request against it.
Later on i want to compare responses with the real api.
"""
import os
import sys
from time import sleep, time
from random import randint
from urllib.request import urlopen
from pathlib import Path


def main():
    api_url = os.getenv("TOKENAPI_URL", None)
    mappings = os.getenv("MAPPINGS", None)
    if not api_url or not mappings:
        sys.exit("Set env vars")

    hashs = [f.stem for f in Path(mappings).glob("*.json")]
    logfile = Path("requests.log").open("a")
    while True:
        n1 = time()
        idx = randint(1, len(hashs))
        subject_hash = hashs[idx]
        response = urlopen(f"{api_url}/metadata/{subject_hash}")
        n2 = time()
        logline = f"{response.status} - {n2-n1} - {subject_hash}"
        logfile.write(logline + "\n")
        print(logline)
        sleep(.1)

if __name__ == '__main__':
    main()