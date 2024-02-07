use std::fmt;


/// Set con creador de Módulos y comparador de estos
pub struct Modulo { 
    pub tipo: String,
    pub dia: String,
    pub modulo: i32,
    pub sala: String
}

impl Modulo {
    const TIPOS: [&'static str; 9] = [  
        "CLAS",
        "LAB",
        "AYU",
        "TAL",
        "LIB",
        "PRA",
        "SUP",
        "TER",
        "TES"];
    
    const DIAS: [&'static str; 6] = [
        "L",
        "M",
        "W",
        "J",
        "V",
        "S"];
    
    const MODULOS: i32 = 8;

    /// Constructor del horario de un curso con tipo, dia, modulo, y sala definidos
    /// 
    /// Siempre y cuando los valores entregados sean los vistos en la página, no ejecutará error.
    pub fn new(tipo: &str, dia: &str, modulo: i32, sala: &str) -> Result<Self, String> {
        if !Self::TIPOS.contains(&tipo) {
            return Err(format!("El tipo ({}) de módulo no es válido", tipo));
        }

        if !Self::DIAS.contains(&dia) && dia != "SIN HORARIO" {
            return Err(format!("El día ({}) del módulo no es válido", dia));
        }

        if (modulo < 1 || Self::MODULOS < modulo) && dia != "SIN HORARIO" {
            return Err(format!(
                "El número ({}) de módulo es inválido. Debe estar entre 1 y {}",
                modulo,
                Self::MODULOS
            ));
        }

        Ok(Modulo {
            tipo: tipo.to_string(),
            dia: dia.to_string(),
            modulo,
            sala: sala.to_string()
        })
    }
    /// Al poner dos cursos distintos, detecta si son similares en cuanto al tipo, día y módulo
    pub fn modulos_identicos(a: &Modulo, b: &Modulo) -> bool {
        a.tipo == b.tipo && a.dia == b.dia && a.modulo == b.modulo
    }
    /// Al poner dos cursos distintos, detecta si pueden complementarse a la hora de hacer el horario
    pub fn modulos_compatibles(a: &Modulo, b: &Modulo) -> bool {
        a.dia != b.dia || a.modulo != b.modulo || a.dia == "SIN HORARIO" || b.dia == "SIN HORARIO"
    }
}

/// Temporal
impl fmt::Display for Modulo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Modulo {{ tipo: {}, dia: {}, modulo: {}, sala: {} }}",
            self.tipo, self.dia, self.modulo, self.sala
        )
    }
}