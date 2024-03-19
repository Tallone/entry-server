# Test not valid id
GET http://localhost:3000/api/v1/user/not_valid
HTTP 200
[Asserts]
jsonpath "$.code" != 0

# Test valid
GET http://localhost:3000/api/v1/user/a39e5954-a827-4467-8b7c-fbbe7fdfa567
HTTP 200
[Asserts]
jsonpath "$.code" == 0
