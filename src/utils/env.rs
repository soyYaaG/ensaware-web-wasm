pub struct Env {
    pub api_ensaware: String,
    pub version: String,
}

impl Env {
    pub fn new() -> Env {
        let env = Env {
            api_ensaware: "https://dev.api.ensaware.yaag.pro".to_string(),
            version: "v1.0.0".to_string(),
        };

        env
    }
}
