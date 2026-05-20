#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;

const PAGE: &str = r#"<!doctype html><html><head><meta charset="utf-8"><title>FlazHost Rocket App</title><style>*{margin:0;padding:0;box-sizing:border-box}body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:linear-gradient(135deg,#1e293b,#0f172a);min-height:100vh;display:flex;align-items:center;justify-content:center;color:#e2e8f0}.card{background:rgba(255,255,255,.05);border:1px solid rgba(255,255,255,.1);border-radius:16px;padding:3rem 2.5rem;text-align:center;max-width:480px}.card h1{font-size:1.6rem;margin-bottom:1rem}.card p{margin-bottom:.6rem;color:#cbd5e1;line-height:1.6}.meta{margin-top:1.5rem;padding-top:1.2rem;border-top:1px solid rgba(255,255,255,.08);font-size:.8rem;color:#64748b}</style></head><body><div class="card"><h1>Rocket berjalan di FlazHost</h1><p>Aplikasi Rocket Anda berhasil di-deploy.</p><p>Edit kode lalu push ke Git untuk redeploy otomatis.</p><div class="meta">Powered by FlazHost Runner</div></div></body></html>"#;

#[get("/")]
fn home() -> RawHtml<&'static str> {
    RawHtml(PAGE)
}

#[get("/health")]
fn health() -> &'static str {
    "{\"status\":\"ok\"}"
}

#[launch]
fn rocket() -> _ {
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(80);
    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));
    rocket::custom(figment).mount("/", routes![home, health])
}
