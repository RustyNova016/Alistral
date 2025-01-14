use musicbrainz_db_lite::models::musicbrainz::relations::Relation;

pub fn is_relation_parent<T, U>(relation: &Relation<T, U>, child_id: i64) -> bool {
    let parent = match relation.direction.as_str() {
        "forward" => parent_entity_forward(&relation.type_id),
        _ => parent_entity_backward(&relation.type_id),
    };

    match parent {
        EntitySide::Entity0 => relation.entity0 == child_id,
        EntitySide::Entity1 => relation.entity1 == child_id,
    }
}

enum EntitySide {
    Entity0,
    Entity1,
}

fn parent_entity_forward(relation_type_id: &str) -> EntitySide {
    match relation_type_id {
        // Lyrical quotation
            "c8283596-6f1f-42db-be9c-def66d387e78"
            // Musical quotaion
            | "c5decae0-535c-4730-aa5f-ab78eadd98ba"=> EntitySide::Entity0,
            _ => EntitySide::Entity1
    }
}

fn parent_entity_backward(relation_type_id: &str) -> EntitySide {
    match parent_entity_forward(relation_type_id) {
        EntitySide::Entity0 => EntitySide::Entity1,
        EntitySide::Entity1 => EntitySide::Entity0,
    }
}
