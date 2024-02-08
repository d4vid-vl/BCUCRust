use reqwest::Client;

pub struct URLS {
        pub cupos: String,
        pub buscacursos: String,
        pub catalogo: String
}

impl URLS {
    /// Muestra los links importantes después de crear una nueva instancia, no requiere parámetros extra
        pub fn new() -> Self {
            URLS {
            cupos: String::from("https://buscacursos.uc.cl/informacionVacReserva.ajax.php"),
            buscacursos: String::from("https://buscacursos.uc.cl/"),
            catalogo: String::from("https://catalogo.uc.cl/index.php")
        }
    }
}

pub async fn get_reqwest(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    Ok(response)
}



