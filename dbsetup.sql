CREATE DATABASE IF NOT EXISTS poggychatserver;
USE poggychatserver;

CREATE TABLE chathistory (
    receiveaddress VARCHAR(255) NOT NULL,
    toaddress VARCHAR(255) NOT NULL,
    msgcontent VARCHAR(8000) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8;