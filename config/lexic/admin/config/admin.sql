BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "groups" (
	"group_id"	VARCHAR(50),
	"group_note"	TEXT,
	PRIMARY KEY("group_id")
);
CREATE TABLE IF NOT EXISTS "users" (
	"user_name"	varchar(20),
	"user_password"	varchar(100),
	"user_email"	varchar(100),
	"user_is_admin"	INTEGER,
	"user_groupes"	TEXT,
	PRIMARY KEY("user_name")
);
INSERT INTO "groups" ("group_id","group_note") VALUES ('admin','Le groupe des administrateurs habilités à gérer les comptes et groupes'),
 ('picsou','Groupe des boursicoteurs amateurs'),
 ('dev','Habilitation pour développer le dictionnaire des applications'),
 ('repas',''),
 ('chinook','Pour accéder à l''application Chinook'),
 ('demo','Groupe invité pour voir la démo de Beedule'),
 ('billerot','Groupe de la famille billerot');
INSERT INTO "users" ("user_name","user_password","user_email","user_is_admin","user_groupes") VALUES ('philippe','$2a$14$pN51Cifuei8ewMKhj/iWuueiL.7FCD/38DqRpznSG7oOvXBYnD6CK','philippe.billerot@gmail.com',1,'admin,billerot,chinook,demo,dev,picsou,repas'),
 ('bnohost','***','bnohost@gmail.com',0,'admin,picsou'),
 ('demo','$2a$14$HcigfBO2Eck098C2nhrcd.ODnQ.w8WT4Yv8dAZv4avYw.pEjwIlZm','demo@crate.fr',0,'chinook,demo'),
 ('Billerot','$2a$14$8JJS3sGVTEBFbIb/A0pRrO/t/wKPQU3bhKC0LteyXfFmXqy2p6Wjm','philippe.billerot@gmail.com',0,'billerot');
COMMIT;
