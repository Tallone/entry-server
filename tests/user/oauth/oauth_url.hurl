GET http://localhost:3000/api/v1/user/oauth/github
[QueryStringParams]
redirect_url: http://localhost:3000/api/v1/user/oauth
HTTP 200
[Asserts]
jsonpath "$.code" == 0

