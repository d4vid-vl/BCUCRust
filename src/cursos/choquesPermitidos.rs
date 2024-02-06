struct Choque {
    sigla1: String,
    tipo1: String,
    sigla2: String,
    tipo2: String,
    permitido: bool
}

impl Choque {
    fn new(sigla1: &str, tipo1: &str, sigla2: &str, tipo2: &str, permitido: bool) -> Self {
        Choque {
            sigla1: sigla1.to_string(),
            tipo1: tipo1.to_string(),
            sigla2: sigla2.to_string(),
            tipo2: tipo2.to_string(),
            permitido,
        }
    }
}

struct ExcepcionMultiplesResultados<'a> {
    resultados: Vec<&'a Choque>, // Tipo de datos adecuado para tus resultados
}

impl ExcepcionMultiplesResultados<'_> {
    fn new(resultados: Vec<&Choque>) -> Self {
        ExcepcionMultiplesResultados { resultados }
    }

    fn to_string(&self) -> String {
        format!("Múltiples choques encontrados. Revisa los choques permitidos o realiza una búsqueda más precisa. Se han encontrado {} resultados.", self.resultados.len())
    }
}

struct ExcepcionSinResultado;

impl ExcepcionSinResultado {
    pub fn new() -> Self {
        ExcepcionSinResultado
    }
}

impl std::fmt::Display for ExcepcionSinResultado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No se encontró ningún choque")
    }
}

enum ChoquesErrors<'a> {
    MultiplesResultados(ExcepcionMultiplesResultados<'a>),
    SinResultado(ExcepcionSinResultado)
}




pub struct ChoquesPermitidos {
    choques: Vec<Choque>
}

impl ChoquesPermitidos {
    pub fn new() -> Self {
        ChoquesPermitidos {
            choques: Vec::new(),
        }
    }
    /// Guarda en el arreglo de choques permitidos, especifico o general, el choque de los módulos de los tipos indicados para las siglas indicadas.
    pub fn add_choque(&mut self, sigla1: &str, tipo1: &str, sigla2: &str, tipo2: &str, permitido: bool) {
        match self.buscar_choque(sigla1, tipo1, sigla2, tipo2) {

            Ok(choque) => { choque.permitido; },
            Err(_) => { self.choques.push(Choque::new(sigla1, tipo1, sigla2, tipo2, permitido)); },
            
        }
    }
    /// Busca un choque de las siglas y los tipos indicados en el arreglo de los choques permitidos.
    /// Si se encuentra más de un choque para las siglas y tipos indicados, levanta una excepción.
    fn buscar_choques(&self, sigla1: &str, tipo1: &str, sigla2: &str, tipo2: &str) -> Result<Vec<&Choque>, ExcepcionMultiplesResultados> {
        
        let choques_encontrados: Vec<&Choque> = self.choques
        .iter()
        .filter(|choque| {

           return ((choque.sigla1 == sigla1 || choque.sigla1 == "*") && (choque.tipo1 == tipo1 || choque.tipo1 == "*") &&
            (choque.sigla2 == sigla2 || choque.sigla2 == "*") && (choque.tipo2 == tipo2 || choque.tipo2 == "*")) ||
            
            ((choque.sigla1 == sigla2 || choque.sigla1 == "*") && (choque.tipo1 == tipo2 || choque.tipo1 == "*") &&
            (choque.sigla2 == sigla1 || choque.sigla2 == "*") && (choque.tipo2 == tipo1 || choque.tipo2 == "*"))

        }).collect();
        
        if choques_encontrados.len() > 1 {

            Err(ExcepcionMultiplesResultados::new(choques_encontrados))

        } else {

            Ok(choques_encontrados)
        }
    }

    /// Busca todos los choques para las siglas y los tipos indicados.
    fn buscar_choque(&self, sigla1: &str, tipo1: &str, sigla2: &str, tipo2: &str) -> Result<&Choque, ChoquesErrors> {

        let choques = self.buscar_choques(sigla1, tipo1, sigla2, tipo2)
            .map_err(|error| match error {
                ExcepcionMultiplesResultados { resultados: err } => ChoquesErrors::MultiplesResultados(ExcepcionMultiplesResultados { resultados:err }),
                other_excepcionsinresultado => ChoquesErrors::SinResultado(ExcepcionSinResultado::new()),
            })?;
    
        if choques.len() > 1 {
            Err(ChoquesErrors::MultiplesResultados(ExcepcionMultiplesResultados::new(choques)))
        } else if choques.is_empty() {
            Err(ChoquesErrors::SinResultado(ExcepcionSinResultado::new()))
        } else {
            Ok(choques[0])
        }
    }
    
    /// Evalua si el choque entre las siglas y los tipos indicados está permitido.
    pub fn evaluar_choque(&mut self, sigla1: &str, tipo1: &str, sigla2: &str, tipo2: &str) -> bool {

        match self.buscar_choque(sigla1, tipo1, sigla2, tipo2) {

            Ok(choque) => choque.permitido,

            Err(_) => false
        }
    }

}
