import httpclient

proc getTopOffers(): string =
  let
    url = "https://plus.three.ie/core/offers/top"
    client = newHttpClient()
  
  result = client.getContent(url)
  return result

when isMainModule:
  echo getTopOffers()
