use ontology_core::{OntologyType, LEVEL_COUNT};

#[test]
fn seven_zones_of_seven_levels() {
    assert_eq!(LEVEL_COUNT, 49);
    for zone in 1..=7 {
        let levels = OntologyType::ALL.iter().filter(|ty| ty.zone() == zone).count();
        assert_eq!(levels, 7, "zone {zone}");
    }
}

#[test]
fn no_duplicate_canonical_types() {
    let mut names = std::collections::HashSet::new();
    for ty in OntologyType::ALL { assert!(names.insert(ty.slug())); }
}
