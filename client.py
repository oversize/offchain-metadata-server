#!/usr/bin/env python3
"""Super simple script to just hammer the api with requests.
For now i just wanted to throw all kinds of valid request against it.

IOGs Api
https://tokens.cardano.org/

CFs API:
https://api.metadata.staging.cf-deployments.org/
https://api.metadata.staging.cf-deployments.org/apidocs


My idea is to write requests and verify that the apis provide the same
output as my rust implementation does.



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

#if not API_URL or not MAPPINGS:
#        sys.exit("Set env vars")

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

    782c158a98aed3aa676d9c85117525dcf3acc5506a30a8d87369fbcb4d6f6e6574
    fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443

    """
    _data = dict({
         "subjects": [
            "986f0548a2fd9758bc2a38d698041debe89568749e20ab9b75a7f4b7444149",
            "d6a8d8af07d704ba941aa1e4095cbb6968e45ccd3e70340867bf9b083138315261726f417a756c47656c6f",
        ],
        "properties": [
            "subject",
            "name",
            "description"
        ]
    })

    #url = "https://api.metadata.staging.cf-deployments.org/mainnet/metadata/query"
    #url = "https://tokens.cardano.org/metadata/query"
    url = "http://127.0.0.1:8081/metadata/query"
    # url = "https://tokens.dev.colo-primary.cf-systems.org/metadata/query"
    print(url)
    rsp = requests.post(url, json=_data)
    print(f"{rsp.status_code} - {rsp.reason} - {rsp.request.headers}")

    if not rsp.status_code == 200:
        return

    data = json.loads(rsp.content)
    print()
    pprint(data)
    return
    # Data must container one key 'subjects' that holds the list of subjects
    # stripped down to the properties that where send
    for s in data["subjects"]:
        pprint(s)
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

def single_subject():
    subject = "fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443"

    url = f"http://127.0.0.1:8081/metadata/{subject}"
    rsp1 = requests.get(url)
    if not rsp1.status_code == 200:
        return
    data1 = json.loads(rsp1.content)

    url = f"https://api.metadata.staging.cf-deployments.org/mainnet/metadata/{subject}"
    rsp2 = requests.get(url)
    if not rsp2.status_code == 200:
        return
    data2 = json.loads(rsp2.content)
    del data2["additionalProperties"]

    assert data1 == data2

    url = f"https://tokens.cardano.org/metadata/{subject}"
    rsp3 = requests.get(url)
    if not rsp3.status_code == 200:
        return
    data3 = json.loads(rsp3.content)

    assert data1 == data3



def main():
    #all_properties()
    #batch_request_subjects()
    # single_property()
    single_subject()


if __name__ == '__main__':
    main()
