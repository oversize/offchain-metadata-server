#!/usr/bin/env python3
"""Super simple script to just hammer the api with requests.
For now i just wanted to throw all kinds of valid request against it.

In order to use this you will  need to have the following packages availble.
This is a rust project so i wanted to keep the python part simple and
just document here. Create a venv and install dependencies:

    python3 -m venv .venv
    source .venv/bin/activate
    pip install reque

"""
import os
import requests
import sys
import json
from pprint import pprint
from socket import timeout
from time import sleep, time
from random import randint
from requests.exceptions import RequestException
from urllib.error import URLError
from pathlib import Path

API_URL = os.getenv("TOKENAPI_URL", None)
MAPPINGS = os.getenv("MAPPINGS", None)

if not API_URL or not MAPPINGS:
        sys.exit("Set env vars")

def all_properties():
    hashs = [f.stem for f in Path(MAPPINGS).glob("*.json")]
    logfile = Path("requests.log").open("a")
    while True:
        try:
            n1 = time()
            idx = randint(1, len(hashs) - 1)
            subject_hash = hashs[idx]
            url = f"{API_URL}/metadata/{subject_hash}"
            response = requests.get(url, timeout=15)
            total = time() - n1
            logline = f"{response.status} - {idx:4} - {total:.6f} - {subject_hash}"
            print(logline)
            logfile.write(logline + "\n")
            #sleep(.01)
        except RequestException as e:
            logline = f"Error {e} - {url}"
            print(logline)
            logfile.write(logline + "\n")
            continue

def batch_request_subjects():
    """Send Post Requests to
    """
    _data = dict({
         "subjects": [
              "782c158a98aed3aa676d9c85117525dcf3acc5506a30a8d87369fbcb4d6f6e6574",
              "fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443"
         ]
    })
    #url = "https://api.metadata.staging.cf-deployments.org/mainnet/metadata/query"
    #url = "https://tokens.cardano.org/metadata/query"
    # url = "http://127.0.0.1:8081/metadata/query"
    url = "https://tokens.dev.colo-primary.cf-systems.org/metadata/query"
    print(url)
    rsp = requests.post(url, json=_data)
    print(f"{rsp.status_code} - {rsp.reason} - {rsp.request.headers}")

    if not rsp.status_code == 200:
        return

    data = json.loads(rsp.content)
    # Data must container one key 'subjects' that holds the list of subjects
    # stripped down to the properties that where send
    for s in data["subjects"]:
        pprint(s.keys())
        print(f'{s["name"]["value"]} -|- {s["description"]["value"]}')

def single_property():
    subjects = [
        "782c158a98aed3aa676d9c85117525dcf3acc5506a30a8d87369fbcb4d6f6e6574",
        "fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443"
    ]

    #url = "https://tokens.cardano.org/XX"
    url = f"https://api.metadata.staging.cf-deployments.org/mainnet/metadata/{subjects[0]}/properties/name"
    print(url)
    rsp = requests.get(url)
    print(f"{rsp.status_code} - {rsp.reason} - {rsp.request.headers}")
    if not rsp.status_code == 200:
        return
    data = json.loads(rsp.content)
    pprint(data)
    # Data must container one key 'subjects' that holds the list of subjects
    # stripped down to the properties that where send

def main():
    #all_properties()
    batch_request_subjects()
    # single_property()


if __name__ == '__main__':
    main()