CREATE DATABASE IF NOT EXISTS searchservice;
DROP USER IF EXISTS 'searchservice'@'%';
CREATE USER 'searchservice'@'%' IDENTIFIED BY 'searchservice';
GRANT ALL PRIVILEGES ON *.* TO 'searchservice'@'%';
FLUSH PRIVILEGES;