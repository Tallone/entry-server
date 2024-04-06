POST http://localhost:3000/api/v1/sync
token: {{token}}
{
    "content": "user's data"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
jsonpath "$.data.version" == 1

GET http://localhost:3000/api/v1/sync
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
jsonpath "$.data.version" == 1


DELETE http://localhost:3000/api/v1/sync
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 0

GET http://localhost:3000/api/v1/sync
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
jsonpath "$.data.version" == 2
