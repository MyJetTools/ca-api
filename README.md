Docker compose example


```yaml

services:
  ca-api:
    image: myjettools/ca-api:0.1.0
    hostname: ca-api
    container_name: ca-api
    restart: always
    deploy:
      resources:
        limits:
          memory: 64Mb
        reservations:
          memory: 64Mb
    volumes:
      - ./.ca-api:/root/.ca-api
      - ./temp:/root/temp
      - ./pki:/usr/share/easy-rsa/pki
    ports:
    - "5959:8000"
    logging:
      options:
        max-size: 512Kb
        max-file: 1
    networks:
    - docker_net

networks:
  docker_net:
    external: true


```
