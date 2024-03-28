POST http://localhost:3000/api/v1/user/oauth/github
{
  "state": "a8498a70-86ef-4088-bbc0-558e7f826306",
  "code": "7ff9a54c22b0670ab73b"
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