FROM centos

RUN yum update -y
RUN yum install -y wget

RUN yum install -y \
       java-1.8.0-openjdk \
       java-1.8.0-openjdk-devel

ENV JAVA_HOME /etc/alternatives/jre

WORKDIR /kafka

# Get kafka
RUN wget https://www-us.apache.org/dist/kafka/2.3.0/kafka_2.12-2.3.0.tgz
RUN tar -xzf kafka_2.12-2.3.0.tgz

WORKDIR /kafka/kafka_2.12-2.3.0

# Move the config file in
COPY ./server.properties config/server.properties

CMD bin/zookeeper-server-start.sh config/zookeeper.properties & bin/kafka-server-start.sh config/server.properties
