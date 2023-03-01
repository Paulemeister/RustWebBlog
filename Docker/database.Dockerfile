FROM mariadb:10.10-rc
COPY ./init.sql /docker-entrypoint-initdb.d/init.sql