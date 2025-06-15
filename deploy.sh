touch wordford.db

docker build -t wordford .
docker run -d \
    -p 8088:3000 \
    -v $(pwd)/wordford.db:/usr/src/wordford/wordford.db \
    --name wordford_container wordford
