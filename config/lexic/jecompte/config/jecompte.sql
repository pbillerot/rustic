
CREATE SEQUENCE IF NOT EXISTS jec_jeux_id_seq INCREMENT 1 MINVALUE 1 START 1;
CREATE TABLE IF NOT EXISTS "public"."jec_jeux" (
    "id" integer DEFAULT nextval('jec_jeux_id_seq') NOT NULL,
    "partie" integer NOT NULL ,
    "joueur" varchar(20) NOT NULL,
    "points" integer DEFAULT 0,
    "rem" varchar(50) DEFAULT '',
    CONSTRAINT "jec_jeux_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

CREATE TABLE IF NOT EXISTS "public"."jec_joueurs" (
    "joueur" varchar(20) PRIMARY KEY NOT NULL,
    "cumul" integer default 0,
    "actif" integer default 0
) WITH (oids = false);

