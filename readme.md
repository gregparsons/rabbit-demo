# RabbitMQ Demo per Lapin

A simple, but realistic demo deploying producer and consumer microservices using rabbitmq, flatbuffers, tokio, 
and strongly-typed messages tagged with uuids.

## Howto
- docker network create rabbit-test
- ```make docker_rabbit```
- ```make docker_build_consumer;make docker_build_producer``` 
- ```make docker_run_consumer```
- ```make_docker_run_producer```

- http://localhost:15672/

## References
- https://docs.rs/lapin/2.2.1/lapin/
- https://hub.docker.com/_/rabbitmq/
- https://flatbuffers.dev/index.html#autotoc_md4