#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::string::ToString;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![world]).launch();
}

#[derive(Debug)]
enum TenantStatus {
    Active,
}

impl ToString for TenantStatus {
    fn to_string(&self) -> String {
        match self {
            TenantStatus::Active => "ACTIVE".into(),
        }
    }
}

#[derive(Debug)]
struct Tenant {
    id: String,
    adminName: String,
    allowCreateTenants: bool,
    storageLimitPerDevice: u64,
    company: String,
    domain: String,
    parent: Box<Tenant>,
    status: String,
}

#[derive(Debug)]
struct Application {
    availablity: bool,
    id: String,
    key: String, // needs revisit
    name: String,
    app_type: String, // needs revisit
    owner: String,    // needs revisit
}
