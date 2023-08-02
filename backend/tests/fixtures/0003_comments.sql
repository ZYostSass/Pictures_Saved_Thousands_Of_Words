-- Delete everything currently in there so we start fresh
DELETE FROM comments;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('comments', 'id'), 1, false);


INSERT INTO comments(content, question_id, answer_id)
    VALUES ('question comment 1', 1, null);

INSERT INTO comments(content, question_id, answer_id)
    VALUES ('question comment 2', 1, null);

INSERT INTO comments(content, question_id, answer_id)
    VALUES ('answer content 1', null, 1);

INSERT INTO comments(content, question_id, answer_id)
    VALUES ('answer content 2', null, 1);


