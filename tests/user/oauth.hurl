GET http://localhost:3000/api/v1/user/oauth/github
HTTP 200
[Asserts]
jsonpath "$.code" == 0

