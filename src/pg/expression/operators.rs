use diesel::pg::Pg;

infix_operator!(SameAs, " ~= ", backend: Pg);
infix_operator!(IsContainedBy, " <@ ", backend: Pg);
