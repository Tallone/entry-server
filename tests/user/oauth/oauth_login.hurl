POST http://localhost:3000/api/v1/user/oauth/github
{
  "state": "ccaae01c-e1fb-454b-a74f-35b415b65a00",
  "code": "2c5b0275d303a50275ec"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0

