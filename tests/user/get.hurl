GET http://localhost:3000/api/v1/user
token: not_valid
HTTP 200
[Asserts]
jsonpath "$.code" != 0

GET http://localhost:3000/api/v1/user
token: uNQlZNtSk8U26EvLzOpONOMj9rnpvGw4dKiUlaYe8ps=
HTTP 200
[Asserts]
jsonpath "$.code" == 0
