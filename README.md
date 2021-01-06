# shodan_api

rusty shodan client

shodan_api is modified heavily from: https://github.com/femiagbabiaka/shodan-rust 

Changes: 
 - Changed from Hyper to reqwest
 - removed form_response function get & post functions return now return body of response.
 - renamed and changed create_http_client to build_client
 - removed request function and added get post explicitly
New Functons: 
 - Added scan post function function