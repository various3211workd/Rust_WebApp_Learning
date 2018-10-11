FROM mysql:5.6

RUN { \
    echo '[mysqld]'; \
    echo 'character-set-server = utf8'; \
} > /etc/mysql/conf.d/charset.cnf

#COPY prepare.sql /docker-entrypoint-initdb.d/prepare.sql

