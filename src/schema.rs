table! {
    doctors (id) {
        id -> Int4,
        name -> Varchar,
        specialization -> Varchar,
    }
}

table! {
    patients (id) {
        id -> Int4,
        first_name -> Varchar,
        second_name -> Varchar,
        phone_number -> Varchar,
        email -> Varchar,
        visit_id -> Int4,
    }
}

table! {
    visits (id) {
        id -> Int4,
        visit_name -> Varchar,
        visit_date -> Varchar,
        doctor_id -> Int4,
    }
}

joinable!(patients -> visits (visit_id));
joinable!(visits -> doctors (doctor_id));

allow_tables_to_appear_in_same_query!(
    doctors,
    patients,
    visits,
);
