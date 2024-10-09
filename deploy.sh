#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
PROJECT_DIR="$(realpath "${BASEDIR}/")"

cd $PROJECT_DIR || exit
echo "Current directory: $(pwd)"

GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
GIT_COMMIT=$(git rev-parse --short HEAD)
IMAGE_TAG="${GIT_BRANCH}-${GIT_COMMIT}"

# countdown timer
function build_docker() {
    IMAGE_NAME=$1
    DOCKERFILE=$2

    echo "Will build image: $IMAGE_NAME:$IMAGE_TAG"

    # rm docker containers and images if exists
    CONTAINERS=$(docker ps -a -q -f name=$IMAGE_NAME)
    if [ -n "$CONTAINERS" ]; then
        docker rm -f $CONTAINERS
    fi
    IMAGES=$(docker images -q $IMAGE_NAME)
    if [ -n "$IMAGES" ]; then
        docker rmi -f $IMAGES
    fi

    # build image
    docker build --progress=plain --no-cache -t $IMAGE_NAME:$IMAGE_TAG -f ./$DOCKERFILE ./

    if [ $? -ne 0 ]; then
        echo "Failed to build image $IMAGE_NAME:$IMAGE_TAG"
        exit 1
    fi
}

build_docker "echo-server" "Dockerfile"

docker tag echo-server:$IMAGE_TAG darklinden/echo-server:$IMAGE_TAG
docker tag echo-server:$IMAGE_TAG darklinden/echo-server:latest

docker push darklinden/echo-server:$IMAGE_TAG
docker push darklinden/echo-server:latest

echo ''
echo "Done!"
