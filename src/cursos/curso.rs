use crate::cursos::Modulo;

pub struct Curso {
    nrc: i32,
    sigla: String,
    permite_retiro: bool,
    ingles: bool,
    seccion: i32,
    aprobacion_espcial: bool,
    area : String,
    formato : String,
    categoria : String,
    nombre : String,
    profesor : String,
    campus: String,
    creditos : i32,
    vacantes_totales : i32,
    vacantes_disponibles : i32,
    horario : Vec<Modulo>
}

impl Curso {

    /// Constructor de un nuevo curso con todas las variables definidas
    pub fn new(nrc: i32,
        sigla: &str,
        permite_retiro: bool,
        ingles: bool,
        seccion: i32,
        aprobacion_espcial: bool,
        area : &str,
        formato : &str,
        categoria : &str,
        nombre : &str,
        profesor : &str,
        campus: &str,
        creditos : i32,
        vacantes_totales : i32,
        vacantes_disponibles : i32,
        horario : Vec<Modulo>) -> Result<Self, String> {
        Ok(Curso {
            nrc,
            sigla: sigla.to_string(),
            permite_retiro,
            ingles,
            seccion,
            aprobacion_espcial,
            area : area.to_string(),
            formato : formato.to_string(),
            categoria : categoria.to_string(),
            nombre : nombre.to_string(),
            profesor : profesor.to_string(),
            campus: campus.to_string(),
            creditos,
            vacantes_totales,
            vacantes_disponibles,
            horario
        })
    }
    /// Al poner dos cursos distintos, detecta si tienen el mismo horario o no
    pub fn mismo_horario(curso1: &Curso, curso2: &Curso) -> bool {
        let horario1 = &curso1.horario;
        let horario2 = &curso2.horario;

        if horario1.len() != horario2.len() { return false }

        for modulo1 in horario1 {
            let mut encontrado = false;
            for modulo2 in horario2 {
                if Modulo::modulos_identicos(modulo1, modulo2) {
                    encontrado = true;
                    break;
                }
            }
            if !encontrado {
                return false;
            }
        }

        for modulo2 in horario2 {
            let mut encontrado = false;
            for modulo1 in horario1 {
                if Modulo::modulos_identicos(modulo2, modulo1) {
                    encontrado = true;
                    break;
                }
            }
            if !encontrado {
                return false;
            }
        }

        true
    }

}

