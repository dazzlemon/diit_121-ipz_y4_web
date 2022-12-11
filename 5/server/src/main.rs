use mysql::*;
use mysql::prelude::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
		let mut app = tide::new();
		app.with(
			tide::sessions::SessionMiddleware::new(
				tide::sessions::MemoryStore::new(),
				b"we reccomend you use std::env::var(\"TIDE_SECRET\").unwrap().as_bytes() instead of a fixed value"
			).with_same_site_policy(cookie::SameSite::None)
		);
		
		app.with(
			tide::security::CorsMiddleware::new()
					.allow_credentials(true)
					.allow_origin(tide::security::Origin::from("http://192.168.31.182:8081"))
		);

		app.with(tide::utils::Before(|mut request: tide::Request<()>| async move {
			let session = request.session_mut();
			let visits: usize = session.get("visits").unwrap_or_default();
			session.insert("visits", visits + 1).unwrap();
			request
		}));

		app.at("/").get(|req: tide::Request<()>| async move {
			let session = req.session();
			let visits: usize = session.get("visits").unwrap();
			println!("{} visited {} times\n", session.id(), visits);
			Ok(format!("{}", visits))
		});

		app.at("/reset")
				.get(|mut req: tide::Request<()>| async move {
					req.session_mut().destroy();
					Ok(tide::Redirect::new("/"))
				});

		app.at("/setName/:firstName/:lastName").get(setName);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn setName(req: tide::Request<()>) -> tide::Result<()> {
	let firstName = req.param("firstName").unwrap();
	let lastName = req.param("lastName").unwrap();
	println!("{} {}", firstName, lastName);
	Ok(())
}

fn setNameDb(sessionId: &str, firstName: &str, lastName: &str)
		-> std::result::Result<(), Box<dyn std::error::Error>> {
	//         mysql:://user:password@host:port/database
	let url = "mysql://root:toor@localhost:3306/myPage";
	let pool = Pool::new(url)?;

	let mut conn = pool.get_conn()?;

	conn.exec(
		r"INSERT INTO myPage.user (sessionId, firstname, lastname)
		                   VALUES (:sessionId, :firstName, :lastName)",
		params! {
			"sessionId" => sessionId,
			"firstName" => firstName,
			"lastName" => lastName,
		}
	)?;

	Ok(())
}

fn getNameDb(sessionId: &str)
		-> std::result::Result<(), Box<dyn std::error::Error>> {
	//         mysql:://user:password@host:port/database
	let url = "mysql://root:toor@localhost:3306/myPage";
	let pool = Pool::new(url)?;

	let mut conn = pool.get_conn()?;

	let name = conn.query_first(
		format!(
			"
				SELECT firstname, lastname
				FROM myPage.usr
				WHERE sessionId = \"{}\"
			",
			sessionId,
		)
	);

	Ok(())
}
