import requests
import ssl

from returns.io import IOSuccess,IOFailure
from returns.result import safe


@safe
def get_top_offers():
    url = "https://plus.three.ie/core/offers/top"
    response = requests.request("GET", url)
    return response.text


if __name__ == "__main__":
    result = get_top_offers()
    match result:
        case IOSuccess(body):
            print(body)
        case _:
            print(result)

    print(ssl.OPENSSL_VERSION)
    print(ssl._ssl.__file__)
