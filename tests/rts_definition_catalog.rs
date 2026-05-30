use fracture_command::rts::{
    command_catalog_for, ALL_BUILDING_KINDS, ALL_RACES, ALL_TECH_KINDS, ALL_UNIT_KINDS,
    BUILDING_DEFINITIONS, RACE_COMMAND_CATALOGS, TECH_DEFINITIONS, UNIT_DEFINITIONS,
};

#[test]
fn every_kind_has_exactly_one_definition() {
    for kind in ALL_UNIT_KINDS {
        assert_eq!(
            1,
            UNIT_DEFINITIONS
                .iter()
                .filter(|definition| definition.kind == *kind)
                .count(),
            "{kind:?} must have exactly one unit definition"
        );
    }

    for kind in ALL_BUILDING_KINDS {
        assert_eq!(
            1,
            BUILDING_DEFINITIONS
                .iter()
                .filter(|definition| definition.kind == *kind)
                .count(),
            "{kind:?} must have exactly one building definition"
        );
    }

    for kind in ALL_TECH_KINDS {
        assert_eq!(
            1,
            TECH_DEFINITIONS
                .iter()
                .filter(|definition| definition.kind == *kind)
                .count(),
            "{kind:?} must have exactly one tech definition"
        );
    }

    for race in ALL_RACES {
        assert_eq!(
            1,
            RACE_COMMAND_CATALOGS
                .iter()
                .filter(|catalog| catalog.race == *race)
                .count(),
            "{race:?} must have exactly one command catalog"
        );
    }
}

#[test]
fn every_unit_has_a_valid_producer_and_requirement() {
    for unit in ALL_UNIT_KINDS {
        let definition = unit.definition();
        let producers: Vec<_> = BUILDING_DEFINITIONS
            .iter()
            .filter(|building| building.produces.contains(unit))
            .collect();

        assert_eq!(
            1,
            producers.len(),
            "{unit:?} should have exactly one producing building"
        );
        assert_eq!(
            definition.race, producers[0].race,
            "{unit:?} should be produced by a building from the same race"
        );

        if let Some(required_tech) = definition.required_tech {
            assert_eq!(
                definition.race,
                required_tech.race(),
                "{unit:?} requires tech from the wrong race"
            );
            assert!(
                BUILDING_DEFINITIONS
                    .iter()
                    .any(|building| building.race == definition.race
                        && building.researches.contains(&required_tech)),
                "{unit:?} requires {required_tech:?}, but no same-race building researches it"
            );
        }
    }
}

#[test]
fn command_catalogs_point_to_valid_race_assets() {
    for race in ALL_RACES {
        let catalog = command_catalog_for(*race);
        let buildings = [
            catalog.supply_building,
            catalog.production_building,
            catalog.advanced_production_building,
            catalog.research_building,
            catalog.ley_claim_building,
            catalog.aether_link_building,
        ];

        for building in buildings {
            assert_eq!(
                *race,
                building.race(),
                "{race:?} catalog points at wrong-race building {building:?}"
            );
        }

        let units = [
            Some(catalog.worker_unit),
            Some(catalog.combat_unit),
            Some(catalog.heavy_unit),
            catalog.specialist_unit,
        ];
        for unit in units.into_iter().flatten() {
            assert_eq!(
                *race,
                unit.race(),
                "{race:?} catalog points at wrong-race unit {unit:?}"
            );
            assert!(
                BUILDING_DEFINITIONS
                    .iter()
                    .any(|building| building.race == *race && building.produces.contains(&unit)),
                "{race:?} catalog unit {unit:?} has no producer"
            );
        }

        let techs = [
            Some(catalog.basic_tech),
            Some(catalog.heavy_tech),
            catalog.specialist_tech,
        ];
        for tech in techs.into_iter().flatten() {
            assert_eq!(
                *race,
                tech.race(),
                "{race:?} catalog points at wrong-race tech {tech:?}"
            );
            assert!(
                BUILDING_DEFINITIONS
                    .iter()
                    .any(|building| building.race == *race && building.researches.contains(&tech)),
                "{race:?} catalog tech {tech:?} has no researcher"
            );
        }
    }
}
