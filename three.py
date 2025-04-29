import requests

def get_top_offers():
    url = "https://plus.three.ie/core/offers/top"
    response = requests.request("GET", url)
    return response.text

if __name__ == "__main__":
    print(get_top_offers())