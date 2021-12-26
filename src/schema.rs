table! {
    answers (id) {
        id -> Uuid,
        parent_id -> Uuid,
        answer -> Text,
    }
}

table! {
    emergencies (id) {
        id -> Uuid,
        active -> Bool,
        owner_id -> Uuid,
        emergency_type -> Varchar,
        created_at -> Varchar,
        helped -> Bool,
    }
}

table! {
    owners (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        country -> Varchar,
        biking_modality -> Varchar,
    }
}

joinable!(answers -> emergencies (parent_id));
joinable!(emergencies -> owners (owner_id));

allow_tables_to_appear_in_same_query!(
    answers,
    emergencies,
    owners,
);
