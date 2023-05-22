# RabbitMQ Demo per Lapin

## Docker
- docker run -d --hostname host-rabbit --name rabbit -e RABBITMQ_DEFAULT_VHOST=my_vhost -p 5672:5672 -p 15672:15672 rabbitmq:3-management
- http://localhost:15672/

## References
- https://docs.rs/lapin/2.2.1/lapin/
- https://hub.docker.com/_/rabbitmq/
