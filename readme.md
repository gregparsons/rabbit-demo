# RabbitMQ Demo per Lapin

Departing from the basic Lapin and flatbuffer howtos this simple demo 
- more realistically models microservices talking to each other; 
- sends a strongly typed message from one to the other;
- serializes a message using flatbuffers (or json);
- stamps each message with a uuid;
- includes the docker run script to run a local broker;

## Howto
- ```docker run -d --rm --hostname host-rabbit --name rabbit -e RABBITMQ_DEFAULT_VHOST=my_vhost -p 5672:5672 -p 15672:15672 rabbitmq:3-management```
- ```cargo run -p producer```
- ```cargo run -p consumer```
- http://localhost:15672/

## References
- https://docs.rs/lapin/2.2.1/lapin/
- https://hub.docker.com/_/rabbitmq/
- https://flatbuffers.dev/index.html#autotoc_md4