curl \
    --write-out ' (HTTP %{response_code})\n' \
    --request POST \
    --url http://127.0.0.1:3000/articles \
    --header 'Content-Type: application/json' \
    --data '{
        "title": "What a fantastic day",
        "content": "Look at all the beautiful flowers",
        "published_date": "2023-08-11"
    }'

id=$(sqlite3 'file:data.db?immutable=1' 'SELECT id FROM articles LIMIT 1;')
curl \
    --write-out ' (HTTP %{response_code})\n' \
    --request GET \
    --url http://127.0.0.1:3000/articles/"${id}"

curl \
    --write-out ' (HTTP %{response_code})\n' \
    --request GET \
    --url http://127.0.0.1:3000/articles/00000000-0000-0000-0000-000000000000

sqlite3 'file:data.db?immutable=1' 'SELECT * FROM articles;'
