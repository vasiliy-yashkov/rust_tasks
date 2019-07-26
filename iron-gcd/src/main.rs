// Use dependencies from Cargo.toml
extern crate iron;
extern crate mime;

extern crate router;
use router::Router; // For events handling

use iron::prelude::*; // Makes opened all names in prelude
use iron::status;

extern crate urlencoded;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

mod gcd;

///
/// An example of simple web server using a web framework iron, 
/// an HTTP server hyper and numerous crate on which they depend. 
/// Site asks the user for two numbers and calculate 
/// their greatest common divisor
fn main() {
    // Create object to route a get and post requests
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    // Print message on how to connect to the server
    println!("Serving on http://localhost:3030...");
    // ... and start listening on 3030 port
    Iron::new(router).http("localhost:3030").unwrap();
}

/// Function to get request
///
/// # Arguments
///
/// * `_request` - HTTP request for processing.
///
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response
        .headers
        .set(iron::headers::ContentType(iron::mime::Mime(
            iron::mime::TopLevel::Text,
            iron::mime::SubLevel::Html,
            vec![(iron::mime::Attr::Charset, iron::mime::Value::Utf8)],
        )));
    response.set_mut(
        r#"
				<title>GCD Calculator</title>
				<form action="/gcd" method="post">
					<input type="text" name="n"/>
					<input type="text" name="n"/>
					<button type="submit">Compute GCD</button>
				</form>
		"#,
    );
    Ok(response)
}

/// Function to post request from form to find GCD
///
/// # Arguments
///
/// * `request` - HTTP request for processing.
///
fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    // Parse the request body and present it as a table
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };
    // bypassing the vector of lines, trying to parse each as a 64-bit number
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    // Find GCD
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd::gcd(d, *m);
    }
    response.set_mut(status::Ok);
    response
        .headers
        .set(iron::headers::ContentType(iron::mime::Mime(
            iron::mime::TopLevel::Text,
            iron::mime::SubLevel::Html,
            vec![(iron::mime::Attr::Charset, iron::mime::Value::Utf8)],
        )));
    response.set_mut(format!(
        "The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d
    ));
    Ok(response)
}
