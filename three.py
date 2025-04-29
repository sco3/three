import requests

def get_top_offers():
    url = "https://plus.three.ie/core/offers/top"

    payload = {}
    headers = {}

    response = requests.request("GET", url, headers=headers, data=payload)

    return response.text


if __name__ == "__main__":
    print(get_top_offers())