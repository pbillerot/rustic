

# docker ps -a | grep silex | awk '{print $1}' | xargs docker rm -f
# docker build --no-cache -t silex .
docker-compose up -d --build
docker image prune -f