GET http://localhost:3000/api/v1/user
token: not_valid
HTTP 200
[Asserts]
jsonpath "$.code" != 0

GET http://localhost:3000/api/v1/user
token: le9ob5tbCMmjXwwfz62qH39a9l55nLBKromwoWj76DY=
HTTP 200
[Asserts]
jsonpath "$.code" == 0
