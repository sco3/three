async function getTopOffers(): Promise<string> {
  const response = await fetch("https://plus.three.ie/core/offers/top");
  if (!response.ok) return Promise.reject();
  return response.text();
}

// Usage
getTopOffers()
  .then(console.log)
  .catch(console.error);
