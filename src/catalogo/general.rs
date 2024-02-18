use scraper::{Html, Selector};
use serde_json::json;
use crate::utils::*;

use self::utils::get_reqwest;

pub async fn obtener_detalles_curso(sigla: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {

    let url = utils::URLS::new().catalogo;

    let response =  get_reqwest(&format!("{}?tmpl=component&view=requisitos&sigla={}", url, sigla)).await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut prerrequisitos = vec![];
    let mut restricciones = String::new();
    let mut relacion = String::new();
    let mut equivalencias = vec![];

    // Prerrequisitos
    let selector_prerrequisitos = Selector::parse("tr:nth-child(1) td:last-child").unwrap();
    let pr = document.select(&selector_prerrequisitos).next().unwrap().text().collect::<String>();
    let prerrequisitos_raw = pr.trim();

    for requisito in prerrequisitos_raw.split('o') {
        let mut cursos = vec![];
        for curso in requisito.split('y') {
            let mut correquisito = false;
            let curso = curso.trim().replace("(c)", "");
            if !curso.is_empty() && curso != " " && curso.contains("c") {
                    correquisito = true;
                
            }

            cursos.push(json!({
                "sigla": curso,
                "correquisito": correquisito
            }));
        }
        prerrequisitos.push(cursos);
    }

    // Relación Prerrequisitos - Restricciones
    let selector_relacion = Selector::parse("tr:nth-child(2) td:last-child").unwrap();
    relacion = document.select(&selector_relacion).next().unwrap().text().collect::<String>().trim().to_string();

    // Restricciones
    let selector_restricciones = Selector::parse("tr:nth-child(3) td:last-child").unwrap();
    restricciones = document.select(&selector_restricciones).next().unwrap().text().collect::<String>().trim().to_string();

    // Equivalencias
    let selector_equivalencias = Selector::parse("tr:nth-child(4) td:last-child").unwrap();
    let er = document.select(&selector_equivalencias).next().unwrap().text().collect::<String>();
    let equivalencias_raw = er.trim();
    equivalencias = equivalencias_raw.replace(|c: char| c == '(' || c == ')', "").split(" o ").map(|s| s.to_string()).collect();

    let resultado = json!({
        "prerrequisitos": prerrequisitos,
        "restricciones": restricciones,
        "relacion": relacion,
        "equivalencias": equivalencias
    });

    Ok(resultado)
}
