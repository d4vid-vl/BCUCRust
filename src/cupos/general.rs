use scraper::{Html, Selector};
use serde_json::json;
use crate::utils::*;


pub async fn obtener_cupos(periodo: &str, nrc: i32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_buscacursos = utils::URLS::new().buscacursos;
    let url_cupos = utils::URLS::new().cupos;
    
    let reqwest = utils::get_reqwest().await?;
    let response = reqwest.get(&format!("{}?nrc={}&termcode={}", url_cupos, nrc, periodo)).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);
    let mut cupos = vec![];

    // Obtener sigla del curso
    let selector_info = Selector::parse("tr:nth-child(3) td:nth-child(4)").unwrap();
    let sigla = document.select(&selector_info).next().unwrap().text().collect::<String>().trim().to_string();

    // Obtener total disponible
    let selector_total = Selector::parse("tr:last-child td:last-child").unwrap();
    let vacantes_disponibles: i32 = document.select(&selector_total).next().unwrap().text().collect::<String>().parse().unwrap_or(0);

    if vacantes_disponibles.is_nan() {
        // No es posible obtener info desde este link,
        // posiblemente curso no tiene vacantes reservadas
        // Obtener info desde buscador de cursos
        let curso = obtener_curso(periodo, nrc, url_buscacursos).await?;

        let cupo = json!({
            "escuela": "Vacantes Libres",
            "nivel": "",
            "programa": "",
            "concentracion": "",
            "cohorte": "",
            "admision": "",
            "vacantesOfrecidas": curso["vacantesTotales"],
            "vacantesOcupadas": curso["vacantesTotales"] - curso["vacantesDisponibles"],
            "vacantesDisponibles": curso["vacantesDisponibles"]
        });

        let resultado = json!({
            "nrc": nrc,
            "sigla": curso["sigla"],
            "vacantesDisponibles": curso["vacantesDisponibles"],
            "cupos": [cupo],
            "inseguro": true
        });

        return Ok(resultado);
    } else {
        // Obtener filas de cupos
        let selector_filas = Selector::parse("tr:not(:first-child):not(:last-child)").unwrap();
        let filas_cupos = document.select(&selector_filas);

        for fila in filas_cupos {
            let columnas = fila.select(&Selector::parse("td").unwrap());

            let escuela = columnas.nth(0).unwrap().text().collect::<String>().trim().to_string();
            let nivel = columnas.nth(0).unwrap().text().collect::<String>().trim().to_string();
            let programa = columnas.nth(0).unwrap().text().collect::<String>().trim().to_string();
            let concentracion = columnas.nth(0).unwrap().text().collect::<String>().trim().to_string();
            let cohorte = columnas.nth(0).unwrap().text().collect::<String>().trim().to_string();
            let admision = columnas.nth(0).unwrap().text().collect::<String>().trim().to_string();
            let vacantes_ofrecidas: i32 = columnas.nth(0).unwrap().text().collect::<String>().parse().unwrap_or(0);
            let vacantes_ocupadas: i32 = columnas.nth(0).unwrap().text().collect::<String>().parse().unwrap_or(0);
            let vacantes_disponibles: i32 = columnas.nth(0).unwrap().text().collect::<String>().parse().unwrap_or(0);

            let cupo = json!({
                "escuela": escuela,
                "nivel": nivel,
                "programa": programa,
                "concentracion": concentracion,
                "cohorte": cohorte,
                "admision": admision,
                "vacantesOfrecidas": vacantes_ofrecidas,
                "vacantesOcupadas": vacantes_ocupadas,
                "vacantesDisponibles": vacantes_disponibles
            });

            cupos.push(cupo);
        }

        let resultado = json!({
            "nrc": nrc,
            "sigla": sigla,
            "vacantesDisponibles": vacantes_disponibles,
            "cupos": cupos,
            "inseguro": false
        });

        return Ok(resultado);
    }
}