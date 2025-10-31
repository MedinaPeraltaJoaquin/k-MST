// src/edge.rs

/// Representa una arista ponderada en un grafo.
#[derive(Debug, Clone)]
pub struct Edge {
    // El peso o costo de la arista.
    weight: f64,
    // El índice del nodo de origen.
    from: usize,
    // El índice del nodo de destino.
    to: usize,
}

impl Edge {
    /// Crea una nueva instancia de `Edge`.
    ///
    /// # Argumentos
    ///
    /// * `weight` - El peso o costo de la arista.
    /// * `from` - El índice del nodo de origen.
    /// * `to` - El índice del nodo de destino.
    pub fn new(weight: f64, from: usize, to: usize) -> Self {
        Edge { weight, from, to }
    }

    /// Obtiene el peso de la arista.
    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    /// Obtiene el índice del nodo de origen.
    pub fn get_from(&self) -> usize {
        self.from
    }

    /// Obtiene el índice del nodo de destino.
    pub fn get_to(&self) -> usize {
        self.to
    }

}

/// Implementación de `Ord` (orden total) para `Edge`.
///
/// **Importante:** El orden está invertido, lo que significa que
/// las aristas con **menor peso** se consideran **mayores** en la ordenación.
/// Esto es útil típicamente para usar `Edge` en un `std::collections::BinaryHeap`
/// (heap de máximo), donde la arista con el menor peso (mayor prioridad) debe
/// ser la primera en extraerse.
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Usa `other.weight.partial_cmp(&self.weight).unwrap()` para invertir
        // la ordenación natural y priorizar pesos menores.
        other.weight.partial_cmp(&self.weight).unwrap()
    }
}

/// Implementación de `PartialOrd` (orden parcial) para `Edge`.
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Implementación de `PartialEq` (igualdad parcial) para `Edge`.
///
/// Dos aristas se consideran iguales si tienen el mismo peso, ignorando
/// los nodos de origen y destino.
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

/// Implementación de `Eq` (igualdad total) para `Edge`.
///
/// Requiere que `PartialEq` se haya implementado de forma consistente.
impl Eq for Edge {}