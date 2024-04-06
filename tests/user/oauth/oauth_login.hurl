POST http://localhost:3000/api/v1/user/oauth/github
{
  "state": "21784311-fa9a-47d2-bcdd-3bfa30384f62",
  "code": "941708fdc2d6e558683b"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
[Captures]
token: jsonpath "$.data"

GET http://localhost:3000/api/v1/user
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 0