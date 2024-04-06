POST http://localhost:3000/api/v1/user
{
    "email": "tallone.shi@outlook.com",
    "password": "$argon2id$v=19$m=19456,t=2,p=1$Q2U0Ao1j+eweMK5JfxkIcA$g7NNa5w0H/Kqf7BCOzv4p9lYVKALPr2ZkREMJvJqPN8",
    "name": "Tallone"
}
HTTP 200
[Asserts]
jsonpath "$.code" == 0
