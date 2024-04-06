POST http://localhost:3000/api/v1/user/login
{
    "email": "tallone.shi@outlook.com",
    "password": "122333"
}
HTTP 200
[Captures]
token: jsonpath "$.data"
[Asserts]
jsonpath "$.code" == 0
