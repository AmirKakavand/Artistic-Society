table! {
    artist (id) {
        id -> Int4,
        name -> Text,
        birthdate -> Nullable<Date>,
    }
}

table! {
    image (id) {
        id -> Int4,
        name -> Nullable<Text>,
        location -> Text,
        artist -> Nullable<Int4>,
    }
}

table! {
    song (id) {
        id -> Int4,
        name -> Text,
        location -> Text,
        artist -> Int4,
        duration -> Int4,
        genre -> Nullable<Text>,
    }
}

joinable!(image -> artist (artist));
joinable!(song -> artist (artist));

allow_tables_to_appear_in_same_query!(artist, image, song,);
