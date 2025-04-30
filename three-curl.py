import pycurl
from io import BytesIO

def get_request(url):
    b = BytesIO()
    c = pycurl.Curl()
    c.setopt(c.URL, url)
    c.setopt(c.WRITEDATA, b)
    try:
        c.perform()
        return b.getvalue().decode(), c.getinfo(c.RESPONSE_CODE)
    except pycurl.error:
        return None, None
    finally:
        c.close()

if __name__ == "__main__":
    r, code = get_request("https://plus.three.ie/core/offers/top")
    print(code, r if r else "Request failed")
