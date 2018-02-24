#!/bin/bash
set -ex

res=$(curl -s -X POST http://0.0.0.0:8000/posts -H "Content-Type: application/json" \
  -d '{"title": "my life", "body": "memes"}')

lastid="$(echo "${res}" | json id)"

# can get it individually
curl -s -X GET "http://0.0.0.0:8000/posts/${lastid}"

# post not published yet
curl -s -X GET http://0.0.0.0:8000/posts | grep -v "${lastid}"

# publish
curl -s -X PUT "http://0.0.0.0:8000/posts/${lastid}"

# post now published
curl -s -X GET http://0.0.0.0:8000/posts | grep "${lastid}"

# delete post
curl -s -X DELETE "http://0.0.0.0:8000/posts/${lastid}"
