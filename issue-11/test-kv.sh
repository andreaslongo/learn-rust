curl \
    --request POST \
    --url http://127.0.0.1:3000/kv/key-1 \
    --header 'Content-Type: application/text' \
    --data 'test content for key-1'

curl \
    --request POST \
    --url http://127.0.0.1:3000/kv/key-2 \
    --header 'Content-Type: application/text' \
    --data 'test content for key-2'

curl \
    --request POST \
    --url http://127.0.0.1:3000/kv/image-1 \
    --header 'Content-Type: image/png' \
    --data-binary @test.png

curl \
    --request GET \
    --url http://127.0.0.1:3000/kv/key-1 \

curl \
    --request GET \
    --url http://127.0.0.1:3000/kv/key-2 \

curl \
    --request GET \
    --url http://127.0.0.1:3000/kv/image-1 \
    > ./test2.png

curl \
    --request GET \
    --url http://127.0.0.1:3000/kv/unknown-key \
