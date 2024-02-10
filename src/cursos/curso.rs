use crate::cursos::Modulo;
use crate::cursos::ChoquesPermitidos;

/// Set de funciones para relacionar cursos y lograr un mejor horario
#[derive(Clone)]
pub struct Curso {
    pub nrc: i32,
    pub sigla: String,
    pub permite_retiro: bool,
    pub ingles: bool,
    pub seccion: i32,
    pub aprobacion_especial: bool,
    pub area : String,
    pub formato : String,
    pub categoria : String,
    pub nombre : String,
    pub profesor : String,
    pub campus: String,
    pub creditos : i32,
    pub vacantes_totales : i32,
    pub vacantes_disponibles : i32,
    pub horario : Vec<Modulo>
}

impl Curso {

    /// Constructor de un nuevo curso con todas las variables definidas
    pub fn new(nrc: i32,
        sigla: &str,
        permite_retiro: bool,
        ingles: bool,
        seccion: i32,
        aprobacion_especial: bool,
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
            aprobacion_especial,
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
    /// Evalua si dos cursos tienen el mismo horario o no
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

    /// Evalua si dos horarios son compatibles según sus módulos o si el choque entre ambos de un tipo especifico lo permite para las siglas correspondientes
    pub fn horarios_compatibles(curso1: &Curso, curso2: &Curso, choques_permitidos: &mut Option<ChoquesPermitidos>) -> bool {
        let horario1 = &curso1.horario;
        let horario2 = &curso2.horario;
        let sigla1 = &curso1.sigla;
        let sigla2 = &curso2.sigla;

        // Comprobar que choques_permitidos no sea None.
        if let Some(choques_permitidos) = choques_permitidos.as_mut() {
        // Dos horarios son compatibles si todos sus módulos son compatibles entre sí,
        // o bien, se permite el choque entre dos módulos de un tipo específico para las siglas correspondientes.
                horario1.iter().all(|modulo1| {
                horario2.iter().all(|modulo2| {
                Modulo::modulos_compatibles(modulo1, modulo2) ||
                choques_permitidos.evaluar_choque(sigla1, &modulo1.tipo, sigla2, &modulo2.tipo)
            })
        })
        } else {
        // Manejar el caso en que choques_permitidos sea None
            false
        }
    }

    /// Crea una nueva instancia de Curso con los argumentos mínimos para poder ser utilizable.
    pub fn curso_minimo(nrc: i32, sigla: &str, seccion: i32, nombre : &str, profesor : &str, vacantes_disponibles : i32, horario : Vec<Modulo>) -> Curso {
        Curso {
            nrc,
            sigla: String::from(sigla),
            permite_retiro: false, // Valor sin definir, se puede dejar así si lo necesesita
            ingles: false, // Valor sin definir, se puede dejar así si lo necesesita
            seccion,
            aprobacion_especial: false, // Valor sin definir, se puede dejar así si lo necesesita
            area: String::new(),
            formato: String::new(),
            categoria: String::new(),
            nombre: String::from(nombre),
            profesor: String::from(profesor),
            campus: String::new(),
            creditos: 0,
            vacantes_totales: 0,
            vacantes_disponibles,
            horario
        }
    }
}

