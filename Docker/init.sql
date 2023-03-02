CREATE DATABASE Paulemeister;

CREATE USER 'webserver'@'%' IDENTIFIED BY 'webserverpassword';



GRANT ALL ON Paulemeister.* TO 'webserver'@'%' IDENTIFIED BY 'webserverpassword';

FLUSH PRIVILEGES;

USE Paulemeister;

CREATE TABLE BlogEntries (
    id int(11) PRIMARY KEY NOT NULL AUTO_INCREMENT,
    heading varchar(255) NOT NULL,
    url varchar(255) NOT NULL UNIQUE,
    content longtext NOT NULL,
    author varchar(50) DEFAULT NULL);