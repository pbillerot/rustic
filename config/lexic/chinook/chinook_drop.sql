/*******************************************************************************
   Chinook Database - Version 1.4
   Script: Chinook_PostgreSql.sql
   Description: Creates and populates the Chinook database.
   DB Server: PostgreSql
   Author: Luis Rocha
   License: http://www.codeplex.com/ChinookDatabase/license
********************************************************************************/
SET SESSION CHARACTERISTICS AS TRANSACTION ISOLATION LEVEL SERIALIZABLE;

DROP TABLE album;
DROP TABLE artist;
DROP TABLE customer;
DROP TABLE employee;
DROP TABLE genre;
DROP TABLE invoiceline;
DROP TABLE invoice;
DROP TABLE mediatype;
DROP TABLE playlist;
DROP TABLE playlisttrack;
DROP TABLE track;

