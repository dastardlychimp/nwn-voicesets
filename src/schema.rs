table! {
    sound (id) {
        id -> Int4,
        id_soundset -> Int4,
        idx_soundset -> Int4,
        name -> Varchar,
        transcription -> Nullable<Varchar>,
        sound_data -> Bytea,
    }
}

table! {
    soundset (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Int4,
        soundset_type -> Int4,
    }
}

table! {
    voiceset (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    voiceset_soundset (id) {
        id -> Int4,
        id_voiceset -> Int4,
        id_soundset -> Int4,
    }
}

joinable!(sound -> soundset (id_soundset));
joinable!(voiceset_soundset -> soundset (id_soundset));
joinable!(voiceset_soundset -> voiceset (id_voiceset));

allow_tables_to_appear_in_same_query!(
    sound,
    soundset,
    voiceset,
    voiceset_soundset,
);
