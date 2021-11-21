# simple auth server

The basic signup sequence is as follows.

```sh
# invite a new user requiring their email
curl -d '{"email": "test@gmail.com"}' -H 'Content-Type: application/json' http://localhost:3000/api/invitation

# todo -> system sends registration email

# user receives email, and clicks registration link
curl --request POST \
     --url http://localhost:3000/api/register/f87910d7-0e33-4ded-a8d8-2264800d1783 \
     --header 'content-type: application/json' \
     --data '{"email":"test@gmail.com", "password":"password"}'
```
