use crate::utils::env::Env;

pub enum RequestPath {
    Authorization,
    Career,
    Permission,
    Profile,
    User,
    Qr,
}

pub fn get_url(request_path: RequestPath, version: String) -> String {
    let env = Env::new();
    let url = format!("{}/{}", env.api_ensaware, version);

    let authorization_path = format!("{}/authorization", url);
    let career_path = format!("{}/career", url);
    let permission_path = format!("{}/permission", url);
    let profile_path = format!("{}/profile", url);
    let user_path = format!("{}/user", url);
    let qr_path = format!("{}/qr", url);

    match request_path {
        RequestPath::Authorization => authorization_path,
        RequestPath::Career => career_path,
        RequestPath::Permission => permission_path,
        RequestPath::Profile => profile_path,
        RequestPath::User => user_path,
        RequestPath::Qr => qr_path,
    }
}
