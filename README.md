# shodan_api

rusty shodan client

shodan_api is modified heavily from: https://github.com/femiagbabiaka/shodan-rust 

Changes: 
 - Changed from Hyper to reqwest
 - removed form_response function get & post functions return now return body of response.
 - renamed and changed create_http_client to build_client
 - removed request function and added get post explicitly
 
New Functions:
 - Scan
 - Scan_status
 - query_for_scan
 - internet
 - scan_dns
 - resolve_dns
 - reverse_dns
