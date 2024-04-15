// @generated automatically by Diesel CLI.

diesel::table! {
    aka_attributes (tconst, ordering, attribute) {
        tconst -> Int4,
        ordering -> Int4,
        attribute -> Varchar,
    }
}

diesel::table! {
    aka_types (tconst, ordering, type_name) {
        tconst -> Int4,
        ordering -> Int4,
        type_name -> Varchar,
    }
}

diesel::table! {
    akas (tconst, ordering) {
        tconst -> Int4,
        ordering -> Int4,
        title -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        language -> Nullable<Varchar>,
        is_original_title -> Nullable<Bool>,
    }
}

diesel::table! {
    crew (tconst, nconst, isdirector) {
        tconst -> Int4,
        nconst -> Int4,
        isdirector -> Bool,
    }
}

diesel::table! {
    genres (tconst, genre) {
        tconst -> Int4,
        genre -> Varchar,
    }
}

diesel::table! {
    names (nconst) {
        nconst -> Int4,
        primaryname -> Varchar,
        birthyear -> Nullable<Int4>,
        deathyear -> Nullable<Int4>,
    }
}

diesel::table! {
    principals (tconst, ordering, nconst) {
        tconst -> Int4,
        ordering -> Int4,
        nconst -> Int4,
        category -> Varchar,
        job -> Nullable<Varchar>,
        characters -> Nullable<Varchar>,
    }
}

diesel::table! {
    titles (tconst) {
        tconst -> Int4,
        titletype -> Nullable<Varchar>,
        primarytitle -> Nullable<Varchar>,
        originaltitle -> Nullable<Varchar>,
        isadult -> Nullable<Bool>,
        startyear -> Nullable<Int4>,
        endyear -> Nullable<Int4>,
        runtimeminutes -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    aka_attributes,
    aka_types,
    akas,
    crew,
    genres,
    names,
    principals,
    titles,
);
