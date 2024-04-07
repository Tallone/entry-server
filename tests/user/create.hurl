POST http://localhost:3000/api/v1/user
{
    "email": "entry_x@entry.com",
    "password": "122333",
    "name": "entry"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
