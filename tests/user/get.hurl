GET http://localhost:3000/api/v1/user
token: not_valid
HTTP 200
[Asserts]
jsonpath "$.code" != 0

GET http://localhost:3000/api/v1/user
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
