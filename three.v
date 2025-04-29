import net.http

fn main() {
	url := 'https://plus.three.ie/core/offers/top'
	response := http.get(url) or {
		eprintln('Request failed: ${err}')
		return
	}
	println(response.body)
}
