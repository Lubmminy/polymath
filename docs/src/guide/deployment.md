# Deployment

While the [CLI](/cli) is useful to test the crawler, in production, you should prefer to deploy a server.
Given that Polymath is a collection of libraries, you can create your own server quite easily.
However, you can use our own server. The documentation below shows you how.

> ⚠️ To have an **API over HTTP** or RPC, you need to **use a customised server**.

## Docker

We recommend that you deploy the polymath crawler using [Docker](https://www.docker.com/) (or Podman). In this example, we're going to deploy polymath with its extension to save pages on **Apache Solr**.

Create a `docker-compose.yaml` and write:

```yaml
services:
    solr:
        image: solr:9-slim
        ports:
            - 8983:8983
        volumes:
            - data:/var/solr
        command:
            - solr-precreate
            - gettingstarted

    zookeeper:
        image: wurstmeister/zookeeper
        ports:
            - 2181:2181

    kafka:
        image: wurstmeister/kafka
        depends_on:
            - zookeeper
        ports:
            - 9092:9092
        environment:
            KAFKA_ADVERTISED_LISTENERS: INSIDE://kafka:9092,OUTSIDE://localhost:9093
            KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: INSIDE:PLAINTEXT,OUTSIDE:PLAINTEXT
            KAFKA_LISTENERS: INSIDE://0.0.0.0:9092,OUTSIDE://0.0.0.0:9093
            KAFKA_INTER_BROKER_LISTENER_NAME: INSIDE
            KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
            KAFKA_CREATE_TOPICS: "baeldung:1:1"


    polymath:
        image: ghcr.io/lubmminy/polymath
        depends_on:
            - solr
            - kafka

volumes:
    data:
```