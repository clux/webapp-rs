#!/bin/bash
set -ex

res=$(curl -X POST http://localhost:8000/posts -H "Content-Type: application/json" \
  -d '{"title": "my life", "body": "memes"}')

lastid="$(echo "${res}" | json id)"

# can get it individually
curl -X GET "http://localhost:8000/posts/${lastid}"
echo

# post not published yet
curl -X GET http://localhost:8000/posts | grep -v "${lastid}"
echo

# publish
curl -X PUT "http://localhost:8000/posts/${lastid}"
echo

# post now published
curl -X GET http://localhost:8000/posts | grep "${lastid}"
echo

# delete post
curl -X DELETE "http://localhost:8000/posts/${lastid}"
echo
