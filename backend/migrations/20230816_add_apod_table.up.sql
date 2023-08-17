CREATE TABLE IF NOT EXISTS apods
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER REFERENCES users ON DELETE CASCADE,
    date        VARCHAR(255) NOT NULL,
    title       VARCHAR(255) NOT NULL,
    explanation TEXT NOT NULL,
    media_type  VARCHAR(50) NOT NULL,
    url         VARCHAR(255) NOT NULL
);