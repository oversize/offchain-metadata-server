#!/usr/bin/env python3
"""
IOGs Api
https://tokens.cardano.org/

CFs API:
https://api.metadata.staging.cf-deployments.org/
https://api.metadata.staging.cf-deployments.org/apidocs
"""
import os
import requests
import json
from pprint import pprint
from socket import timeout
from time import sleep, time
from datetime import datetime
from random import randint
from requests.exceptions import RequestException
from urllib.error import URLError
from pathlib import Path

API_URL = os.getenv("TOKENAPI_URL", None)
MAPPINGS = os.getenv("MAPPINGS", None)

ALL_HASHS = [f.stem for f in Path(MAPPINGS).glob("*.json")]
# ALL_HASHS = [f.stem for f in Path("/home/msch/src/cf/cardano-token-registry/mappings").glob("*.json")]


def all_properties():
    logfile = Path("requests.log").open("a")
    url = ""
    while True:

        try:
            idx = randint(1, len(ALL_HASHS) - 1)
            subject_hash = ALL_HASHS[idx]
            url = f"https://tokens.cardano.org/metadata/{subject_hash}"
            response_original = requests.get(url, timeout=5)
            if not response_original.status_code == 200:
                print(f"Got {response_original.status_code} from {url}")
                continue
            data1 = json.loads(response_original.content)

            # url = f"http://TokenApiRS-772077917.eu-central-1.elb.amazonaws.com/metadata/{subject_hash}"
            # url = f"https://tzfsnrjw7xwodrfatlft5a6zia0meaxo.lambda-url.eu-central-1.on.aws/metadata/{subject_hash}"
            # url = f"http://OffchainMetadataLambda-450124955.eu-central-1.elb.amazonaws.com/metadata/{subject_hash}"
            url = f"http://127.0.0.1:8080/metadata/{subject_hash}"
            response_us = requests.get(url, timeout=5)
            logline = f"{datetime.now()}  "
            if not response_us.status_code == 200:
                print(f"Got {response_us.status_code} from {url}")
                continue
            data2 = json.loads(response_us.content)

            if data1 != data2:
                msg = f"Not Equal - {subject_hash}"
                logline += msg
                print(msg)
            else:
                msg = f"Equal {subject_hash}"
                print(msg)
                logline += msg
                assert data1 == data2
            sleep(.01)
            logfile.write(logline + "\n")
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
            "782c158a98aed3aa676d9c85117525dcf3acc5506a30a8d87369fbcb4d6f6e6574",
            "fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443"
        ],
        #"properties": [
            # "subject",
            # "name",
            # "description",
        #]
    })

    #url = "https://api.metadata.staging.cf-deployments.org/mainnet/metadata/query"
    #url = "https://tokens.cardano.org/metadata/query"
    url = "http://127.0.0.1:8080/metadata/query"
    # url = "https://tzfsnrjw7xwodrfatlft5a6zia0meaxo.lambda-url.eu-central-1.on.aws/metadata/query"
    # url = "https://tokens.dev.colo-primary.cf-systems.org/metadata/query"

    rsp = requests.post(url, json=_data)
    print(f"{rsp.status_code} - {rsp.reason} - {url}")

    if not rsp.status_code == 200:
        return

    data = json.loads(rsp.content)
    pprint(data)

    return
    # Data must container one key 'subjects' that holds the list of subjects
    # stripped down to the properties that where send
    for s in data["subjects"]:
        pprint(s)
        print(f'{s["name"]["value"]} -|- {s["description"]["value"]}')

def single_property():
    subjects = [
        "5c4f08f47124b8e7ce9a4d0a00a5939da624cf6e533e1dc9de9b49c5556e636c6542656e6e793630",
        "782c158a98aed3aa676d9c85117525dcf3acc5506a30a8d87369fbcb4d6f6e6574",
        "fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443"
    ]

    #url = "https://tokens.cardano.org/XX"
    url = f"http://127.0.0.1:8080/metadata/{subjects[0]}/properties/name"
    # url = f"https://api.metadata.staging.cf-deployments.org/mainnet/metadata/{subjects[0]}/properties/name"
    rsp = requests.get(url)
    if not rsp.status_code == 200:
        return
    data = json.loads(rsp.content)
    pprint(data)
    # Data must container one key 'subjects' that holds the list of subjects
    # stripped down to the properties that where send

def single_subject():
    subject = "fc4c6a1f2b159e3ea03259286de2061b8d3bc8d42dfb8a6105c5a9904357425443"
    url = f"http://127.0.0.1:8080/metadata/{subject}"
    #url = f"http://ec2-18-193-66-224.eu-central-1.compute.amazonaws.com/metadata/{subject}"
    rsp1 = requests.get(url)
    if not rsp1.status_code == 200:
        pprint(rsp1)
        return
    data1 = json.loads(rsp1.content)
    print(data1)


def main():
    #all_properties()
    batch_request_subjects()
    print()
    single_property()
    #single_subject()

if __name__ == '__main__':
    main()
