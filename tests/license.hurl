GET http://localhost:3000/api/v1/license/test_license
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 4300

POST http://localhost:3000/api/v1/license/test_license
token: {{token}}
{
    "device_id": "this is a device_id"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
jsonpath "$.data.is_used" == true

GET http://localhost:3000/api/v1/license/test_license
token: {{token}}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
