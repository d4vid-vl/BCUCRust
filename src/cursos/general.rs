use crate::cursos::curso::*;
use crate::cursos::modulo::*;
use crate::utils::*;

use reqwest::Client;
use scraper::Html;
use scraper::Selector;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

async fn obtener_cursos(url: &str) -> Result<Vec<Curso>, Box<dyn std::error::Error + Send + Sync>> {
    let response = utils::get_reqwest(url).await?;
    let html = response.text().await?;
    let mut cursos = Vec::new();

    let document = Document::from(html.as_str());

    for node in document.find(Class("resultadosRowPar").or(Class("resultadosRowImpar"))) {
        let mut columnas = Vec::new();

        for td in node.find(Name("td")) {
            columnas.push(td);
        }

        let nrc = columnas[0].text().trim().parse::<i32>()?;
        let sigla = columnas[1].text().trim().to_string();
        let permite_retiro = columnas[2].text().trim() == "SI";
        let ingles = columnas[3].text().trim() == "SI";
        let seccion = columnas[4].text().trim().parse::<i32>()?;
        let aprobacion_especial = columnas[5].text().trim() == "SI";
        let area = columnas[6].text().trim().to_string();
        let formato = columnas[7].text().trim().to_string();
        let categoria = columnas[8].text().trim().to_string();
        let nombre = columnas[9].text().trim().to_string();
        let profesor = columnas[10].text().trim().to_string();
        let campus = columnas[11].text().trim().to_string();
        let creditos = columnas[12].text().trim().parse::<i32>()?;
        let vacantes_totales = columnas[13].text().trim().parse::<i32>()?;
        let vacantes_disponibles = columnas[14].text().trim().parse::<i32>()?;

        let mut horario = Vec::new();
        let horario_filas = columnas[16].find(Name("tr"));

        for fila_horario in horario_filas {
            let columnas_horario = fila_horario.find(Name("td"));

            let mut horas = String::new();
            let mut tipo = String::new();
            let mut sala = String::new();
            for (i, columna) in columnas_horario.enumerate() {
                if i == 0 {
                    horas = columna.text();
                    break;
                } else if i == 1 {
                    tipo = columna.text().trim().to_string();
                    break;
                } else if i == 2 {
                    sala = columna.text().trim().to_string();
                break;
                }       
            }
            let horas_modulos = horas.trim().split(':').collect::<Vec<_>>();
            let dias = horas_modulos[0].split('-').map(|dia| dia.trim()).map(|dia| if dia.is_empty() { "SIN HORARIO" } else { dia }).collect::<Vec<_>>();
            let modulos = horas_modulos[1].split(',').map(|modulo| modulo.trim().parse::<i32>().unwrap_or(0)).collect::<Vec<_>>();

            for dia in dias {
                for modulo in &modulos {
                    horario.push(Modulo::new(&tipo, &dia, *modulo, &sala)?);
                }
            }
        }
        cursos.push(Curso::new(nrc, &sigla, permite_retiro, ingles, seccion, aprobacion_especial, &area, &formato, &categoria, &nombre, &profesor, &campus, creditos, vacantes_totales, vacantes_disponibles, horario)?);
    }

    Ok(cursos)

}

pub async fn buscar_sigla(periodo: &str, sigla: &str) -> Result<Vec<Curso>, Box<dyn std::error::Error + Send + Sync>> {
    let url = utils::URLS::new().buscacursos;
    let url_completa = format!("{}?cxml_semestre={}&cxml_sigla={}", url, periodo, sigla);
    let resultado = obtener_cursos(&url_completa).await;

    return resultado;
}

pub async fn buscar_profesor(periodo: &str, profesor: &str) -> Result<Vec<Curso>, Box<dyn std::error::Error + Send + Sync>> {
    let url = utils::URLS::new().buscacursos;
    let url_completa = format!("{}?cxml_semestre={}&cxml_profesor={}", url, periodo, profesor);
    let resultado = obtener_cursos(&url_completa).await;

    return resultado;
}

pub async fn buscar_curso(periodo: &str, nombre: &str) -> Result<Vec<Curso>, Box<dyn std::error::Error + Send + Sync>> {
    let url = utils::URLS::new().buscacursos;
    let url_completa = format!("{}?cxml_semestre={}&cxml_nombre={}", url, periodo, nombre);
    let resultado = obtener_cursos(&url_completa).await;

    return resultado;
}

pub async fn obtener_curso(periodo: &str, nrc: i32) -> Result<Curso, Box<dyn std::error::Error + Send + Sync>> {
    let url = utils::URLS::new().buscacursos;
    let url_completa = format!("{}?cxml_semestre={}&cxml_nrc={}", url, periodo, nrc);
    let resultado = obtener_cursos(&url_completa).await?;

    if let Some(curso) = resultado.first() {
        Ok(curso.clone())
    } else {
        Err("Número de Curso no válido".into())
    }
}

pub async fn obtener_periodos() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let url = utils::URLS::new().buscacursos;
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let selector = Selector::parse("select[name=\"cxml_semestre\"] option").unwrap();
    let options = document.select(&selector);

    let mut periodos = Vec::new();
    for option in options {
        let text = option.text().collect::<String>().trim()
            .replace(" Primer Semestre", "-1")
            .replace(" Segundo Semestre", "-2")
            .replace(" TAV", "-3");
        periodos.push(text);
    }

    Ok(periodos)
}
