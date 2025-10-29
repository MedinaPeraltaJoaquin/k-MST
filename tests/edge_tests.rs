use k_mst::entity::edge::Edge;

#[test]
/// Prueba la creación de una nueva arista y la corrección de sus campos.
fn test_new_and_getters() {
    let weight = 15.3;
    let from = 0;
    let to = 5;

    let edge = Edge::new(weight, from, to);

    // Verifica que los getters devuelvan los valores correctos
    assert_eq!(edge.get_weight(), weight, "El peso debe ser el asignado.");
    assert_eq!(edge.get_from(), from, "El nodo de origen debe ser el asignado.");
    assert_eq!(edge.get_to(), to, "El nodo de destino debe ser el asignado.");
}

#[test]
/// Prueba la función `get_tuple` para verificar que devuelve la tupla correcta.
fn test_get_tuple() {
    let edge = Edge::new(2.5, 1, 9);
    let expected_tuple = (1, 9, 2.5);
    
    // Verifica que la tupla devuelta coincida con los valores.
    assert_eq!(edge.get_tuple(), expected_tuple, "La tupla debe contener (from, to, weight).");
}

#[test]
/// Prueba la implementación de `PartialEq` y `Eq`.
///
/// Dos aristas son iguales si sus pesos son iguales, independientemente de `from` y `to`.
fn test_partial_eq_and_eq() {
    // Misma arista
    let edge1 = Edge::new(10.0, 1, 2);
    let edge2 = Edge::new(10.0, 1, 2);
    assert_eq!(edge1, edge2, "Aristas idénticas deben ser iguales.");

    // Mismo peso, diferentes nodos
    let edge3 = Edge::new(10.0, 5, 8);
    assert_eq!(edge1, edge3, "Aristas con el mismo peso deben ser iguales (solo se compara el peso).");
    
    // Diferente peso
    let edge4 = Edge::new(10.1, 1, 2);
    assert_ne!(edge1, edge4, "Aristas con diferente peso no deben ser iguales.");
}

#[test]
/// Prueba la implementación invertida de `Ord` y `PartialOrd` para priorizar
/// aristas con **menor** peso.
fn test_ord_and_partial_ord_inverted() {
    // edge_small_weight tiene el menor peso.
    let edge_small_weight = Edge::new(5.0, 0, 1); 
    // edge_medium_weight tiene un peso intermedio.
    let edge_medium_weight = Edge::new(10.0, 1, 2);
    // edge_large_weight tiene el mayor peso.
    let edge_large_weight = Edge::new(20.0, 2, 3);
    
    // 1. Prueba de que menor peso es "mayor" en la ordenación (es decir, mayor prioridad)
    // Edge_small_weight debe ser "mayor" que edge_medium_weight (se compara con self > other)
    assert!(edge_small_weight > edge_medium_weight, "Menor peso debe ser considerado 'mayor' para la prioridad.");
    
    // 2. Prueba de ordenación
    let mut edges = vec![
        edge_large_weight.clone(),
        edge_small_weight.clone(),
        edge_medium_weight.clone(),
    ];
    
    // La ordenación normal de `Vec::sort` usa `Ord::cmp` (de menor a mayor)
    // Como `cmp` está invertido, ordenará de **mayor peso a menor peso**.
    // Por lo tanto, el orden de menor a mayor prioridad será:
    // [edge_large_weight, edge_medium_weight, edge_small_weight]
    edges.sort(); 

    // Verificamos el orden resultante (mayor peso a menor peso)
    assert_eq!(edges[0].get_weight(), 20.0, "La arista con mayor peso debe ser la primera.");
    assert_eq!(edges[1].get_weight(), 10.0, "La arista con peso intermedio debe ser la del medio.");
    assert_eq!(edges[2].get_weight(), 5.0, "La arista con menor peso debe ser la última.");


    // 3. Prueba para BinaryHeap (que usa Max-Heap)
    // El BinaryHeap extrae el elemento "mayor" según Ord (mayor prioridad = menor peso)
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(edge_large_weight);
    heap.push(edge_small_weight);
    heap.push(edge_medium_weight);

    // El primer elemento extraído debe ser el de menor peso (mayor prioridad)
    assert_eq!(heap.pop().unwrap().get_weight(), 5.0, "El BinaryHeap debe extraer primero la arista de menor peso.");
    assert_eq!(heap.pop().unwrap().get_weight(), 10.0, "El siguiente debe ser el de peso intermedio.");
    assert_eq!(heap.pop().unwrap().get_weight(), 20.0, "El último debe ser el de mayor peso.");
}