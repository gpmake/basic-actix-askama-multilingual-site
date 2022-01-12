use actix_web::{http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "home/home.html", localizer = "lang")]
#[l10n(r#""en""#, "home/home-en.html")]
#[l10n(r#""it""#, "home/home-it.html")]
struct Home<'a> {
    lang: &'a str,
    title: &'a str,
    page: &'a str,
}

#[derive(Template)]
#[template(path = "about/about.html", localizer = "lang")]
#[l10n(r#""en""#, "about/about-en.html")]
#[l10n(r#""it""#, "about/about-it.html")]
struct About<'a> {
    lang: &'a str,
    title: &'a str,
    page: &'a str,
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::TemporaryRedirect()
        .insert_header((http::header::LOCATION, "/en"))
        .finish())
}

async fn home(req: HttpRequest) -> Result<HttpResponse> {
    let lang: String = req.match_info().get("lang").unwrap().parse().unwrap();
    if lang == "en" || lang == "it" {
        let s = Home {
            lang: &lang,
            title: &format!("Home-{}", &lang),
            page: &"home".to_string(),
        }
        .render()
        .unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        Ok(HttpResponse::TemporaryRedirect()
            .insert_header((http::header::LOCATION, "/en"))
            .finish())
    }
}

async fn about(req: HttpRequest) -> Result<HttpResponse> {
    let lang: String = req.match_info().get("lang").unwrap().parse().unwrap();
    if lang == "en" || lang == "it" {
        let s = About {
            lang: &lang,
            title: &format!("Home-{}", &lang),
            page: &"about".to_string(),
        }
        .render()
        .unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        Ok(HttpResponse::TemporaryRedirect()
            .insert_header((http::header::LOCATION, "/en"))
            .finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{lang}").route(web::get().to(home)))
            .service(web::resource("/{lang}/about").route(web::get().to(about)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
