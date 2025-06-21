touch wordford.db

docker build -t wordford .
docker run -d \
    -p 8088:3000 \
    -v $(pwd)/wordford.db:/usr/src/wordford/wordford.db \
    -v $(pwd)/templates:/usr/src/wordford/templates \
    --name wordford_container wordford
