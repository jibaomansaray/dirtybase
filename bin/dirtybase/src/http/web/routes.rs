use actix_web::Scope;

pub fn register_routes(scope: Scope) -> Scope {
    println!("do registration");

    scope
}
