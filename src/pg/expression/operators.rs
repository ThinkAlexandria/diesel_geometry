use diesel::pg::Pg;

diesel_infix_operator!(SameAs, " ~= ", backend: Pg);
