# Makefile
docker_build_producer:
	docker build -f docker_file/dockerfile_produce -t produce .
docker_build_consumer:
	docker build -f docker_file/dockerfile_consume -t consumer .
docker_run_producer:
	docker run --rm --net rabbit-net -e AMQP_ADDR="amqp://host-rabbit/my_vhost" --name producer producer
docker_run_consumer:
	docker run --rm --net rabbit-net -e AMQP_ADDR="amqp://host-rabbit/my_vhost" --name consumer consumer
docker_rabbit:
	docker run -d --net rabbit-net --rm --hostname host-rabbit --name rabbit -e RABBITMQ_DEFAULT_VHOST=my_vhost -p 15672:15672 rabbitmq:3-management

