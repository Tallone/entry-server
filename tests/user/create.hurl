POST http://localhost:3000/api/v1/user
{
    "email": "tallone.shi@outlook.com",
    "password": "$argon2id$v=19$m=19456,t=2,p=1$q7B6QeFR0inm+oB1oBuAYQ$WioZWSYdwUNqvLeYm1dDiq+vEt4Xfbd1XH3yE28un3E",
    "hash": "q7B6QeFR0inm+oB1oBuAYQ",
    "name": "Tallone"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
