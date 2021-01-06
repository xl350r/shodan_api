/// shodan provides an ergonomic inteface for interacting with the shodan.io REST API.
/// # Examples
/// ```rust
/// extern crate shodan;
/// fn main() {
/// use shodan::shodan::ShodanClient;
/// let response = ShodanClient::new("INSERT_API_KEY")
///  	.host_info("INSERT_IP_ADDRESS");
/// }
/// ```
/// For more information on the Shodan API, visit [this link](https://developer.shodan.io/api).
extern crate reqwest;

pub mod shodan {
	use reqwest::blocking::Client;

    /// BaseUrl is the basis for all of our api requests.
    const BASE_URL: &'static str = "https://api.shodan.io";

    pub struct ShodanClient {
		api_key: &'static str,
    }

    impl ShodanClient {
		fn build_client(&self) -> Client {
			Client::new()
		}
		fn post(&self, url: String, params: Vec<(String, String)>) -> Result<String, String> {
			let client = self.build_client();
			let response = client.post(&url).form(&params).send().unwrap();
			match &response.status().is_success() {
				true => {Ok(response.text().unwrap())},
				false => {Err(response.text().unwrap())}
			}
		}
		fn get(&self, url: String) -> Result<String, String> {
			let client = self.build_client();
			let response = client.get(&url).send().unwrap();
			match &response.status().is_success() {
				true => {Ok(response.text().unwrap())},
				false => {Err(response.text().unwrap())}
			}
		}

		/// Creates a new ShodanClient.
		pub fn new(api_key: &'static str) -> ShodanClient {
			ShodanClient { api_key: api_key }
		}

		/// Method for `/shodan/host/{ip}`
		pub fn host_info(&self, ip_address: &str) -> Result<String, String> {
			let formatted_url = format!("{}/shodan/host/{}?key={}",
						BASE_URL,
						ip_address,
						self.api_key);

			let response = self.get( formatted_url);
			response
		}

		/// Method for `/shodan/host/search`, no facets.
		pub fn search(&self, query: &str) -> Result<String, String> {
			let formatted_url = format!("{}/shodan/host/search?key={}&query={}",
						BASE_URL,
						self.api_key,
						query);

			let response = self.get( formatted_url);
			response
		}

		/// Method for `/shodan/host/search`, with facets.
		pub fn search_with_facets(&self, query: &str, facets: &str) -> Result<String, String> {
			let formatted_url = format!("{}/shodan/host/search?key={}&query={}&facets={}",
						BASE_URL,
						self.api_key,
						query,
						facets);

			let response = self.get( formatted_url);
			response
		}

		/// Method for `/labs/honeyscore/{ip}`.
		pub fn honeyscore(&self, ip_address: &str) -> Result<String, String> {
			let formatted_url = format!("{}/labs/honeyscore/{}?key={}",
						BASE_URL,
						ip_address,
						self.api_key);

			let response = self.get( formatted_url);
			response
		}
		pub fn protocols(&self) -> Result<String, String> {
			let formatted_url = format!("{}/shodan/protocols?key={}",
						BASE_URL,
						self.api_key);

			let response = self.get( formatted_url);
			response
		}
		pub fn scan(&self, ip_addresses: &str) -> Result<String, String> {
			let formatted_url = format!("{}/shodan/scan?key={}",
				BASE_URL,
				self.api_key
			 );
			let params =vec![("ip".to_string(), ip_addresses.to_string())];
			let response = self.post(formatted_url, params);
			response
		}
		pub fn scan_status(&self, id: &str) -> Result<String, String>  {
			let formatted_url = format!("{}/shodan/scan/{}?key={}",
						BASE_URL,
						id,
						self.api_key);
			let response = self.get(formatted_url);
			response
		}
		pub fn query_for_scan(&self, id: &str) -> Result<String, String> {
			let query=format!("scan:{}", id);
			let response = self.search(&query);
			response
		}
		//post
		pub fn internet(&self, port: i32, protocol: &str) -> Result<String, String>  {
			let formatted_url = format!("{}/shodan/scan/internet?key={}",
				BASE_URL,
				self.api_key
			 );
			let params =vec![("port".to_string(), port.to_string()), ("protocol".to_string(), protocol.to_string())];
			let response = self.post(formatted_url, params);
			response
			
		}
		pub fn scan_dns(&self, domain: &str) -> Result<String, String> {
			let formatted_url = format!("{}/dns/domain/{}?key={}",
				BASE_URL,
				domain,
				self.api_key);
			let response = self.get(formatted_url);
			response
		}
		pub fn resolve_dns(&self, hostnames: &str) -> Result<String, String> {
			let formatted_url = format!("{}/dns/resolve?hostnames={}&key={}",
				BASE_URL,
				hostnames,
				self.api_key
				);
			let response = self.get(formatted_url);
			response
		}
		pub fn reverse_dns(&self, ips: &str) -> Result<String, String> {
			let formatted_url  = format!("{}/dns/reverse?ips={}&key={}", 
				BASE_URL,
				ips,
				self.api_key
				);
			let response = self.get(formatted_url);
			response
		}
    }
}

