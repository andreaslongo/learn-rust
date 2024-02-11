curl \
    --request POST \
    --url http://127.0.0.1:3000/user \
    --header 'Content-Type: application/json' \
    --data '{
        "name": "Testuser",
        "email": "your@mail.com",
        "age": 19
    }'


curl \
    --request POST \
    --url http://127.0.0.1:3000/user \
    --header 'Content-Type: application/json' \
    --data '{
        "name": "Testuser",
        "email": "invalid email",
        "age": 19
    }'
