CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  username TEXT NOT NULL,
  nick TEXT,
  updated TEXT NOT NULL,
  current_status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
  id TEXT NOT NULL PRIMARY KEY,
  chat_id INTEGER NOT NULL,
  users_count INTEGER DEFAULT 0,
  group_name TEXT,
  is_active INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS custom_groups (
  creater_id TEXT NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  users_count INTEGER DEFAULT 0,
  group_name TEXT,
  is_active INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS groups_users (
  chat_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  PRIMARY KEY(chat_id, user_id)
);


insert OR IGNORE into users (id, name, nick, updated, status) VALUES (123, 'Мфдпфдф_1', NULL, '2022-12-12T12:00:00', 'plus');
insert OR IGNORE into users (id, name, nick, updated, status) VALUES (1234, 'Мфдпфдф_2', 'Вфявка', '2022-12-12T13:00:00', 'plus');
insert OR IGNORE into users (id, name, nick, updated, status) VALUES (12345, 'Мфдпфдф_3', 'Вфявка_2', '2022-12-12T14:00:00', 'plus');
insert OR IGNORE into users (id, name, nick, updated, status) VALUES (123456, 'Мфдпфдф_4', 'Вфявка_3', '2022-12-12T15:00:00', 'plus');
SELECT * FROM users;

INSERT INTO groups  (id, chat_id, users_count, name, is_active) VALUES ('guid_2', 124123, 5, NULL, 0)
select * from groups

insert into groups_users (chat_id, user_id) values (-312124123, 123456)

SELECT u.id,  u.username, u.nick,  u.updated, u.current_status,  gr.users_count
FROM chat_id_user_id AS cu
LEFT JOIN users AS u ON cu.user_id = u.id
LEFT JOIN groups AS gr ON gr.chat_id = cu.chat_id
WHERE  cu.chat_id = -312124123
