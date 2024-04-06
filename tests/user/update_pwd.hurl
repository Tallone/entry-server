PATCH http://localhost:3000/api/v1/user/password
token: {{token}}
{
    "new_password": "122333"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0