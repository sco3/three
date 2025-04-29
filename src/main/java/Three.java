import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class Three {

	public static String getTopOffers() throws IOException, InterruptedException {
		String url = "https://plus.three.ie/core/offers/top";

		HttpClient client = HttpClient.newHttpClient();

		HttpRequest request = HttpRequest.newBuilder().uri(URI.create(url)).GET().build();

		HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

		return response.body();
	}

	public static void main(String[] args) throws Exception {
		System.out.println(getTopOffers());
	}
}
