curl --request POST \
  --url http://127.0.0.1:3000/articles \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "What a fantastic day",
	"content": "Look at all the beautiful flowers",
	"published_date": "2023-08-11"
}'

curl --request GET \
  --url http://127.0.0.1:3000/articles/1
