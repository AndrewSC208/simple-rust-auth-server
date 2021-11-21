# Simple Auth Server
A simple actix server that can be used for user invitation, rregistration, login, and logout. It's designed to
be really simple mainly as an exercise to learn `rust`, and `actix`. This is not an asyncrounus service
that was outside of the scope of this project. The database is `posgres` and the schema is managed with `Diesel`.

In the future I will add some more api endpoints to drive my learning deeper, but at this time i'm not really working
on this project anymore.

## Getting started
Running the system is very easy, make sure you have `rust`, `cargo-watch` and `Docker` installed. Once all dependencies
are installed run `$ make local`. This will run postgres, migrate the db, and run cargo watch on the server. After the system
is running locally testing the user functionality is also pretty straght forward. It's just a few make targets.

## Testing Functionality
To kick off the user workflow we need to invite a user to register with the system. You do that with the 
`invite-user` target. Don't forget to provid an testing email.
```
$ make invite-user EMAIL=test@gmail.com
```

Get the `invitation_id` from the database, or the server logs and register the user with the `invitation_id` by running
the `register-user` command.
```
$ make register-user \
  INVITATION_ID=<ID_FROM_DB> \
  EMAIL=test@gmail.com \
  PASSWORD=test
```

Now that your user is registered. Let's login.
```
$ make login-user \
  EMAIL=test@gmail.com \
  PASSWORD=test
```
Once you get a 200, a cookie should be output to standard out and that can be used to test authenticated routes.

Run the below command to test authentication on protected routes.
```
$ make test-auth \
  COOKIE=<TOKEN FROM LOGIN COMMAND>
```

Now that you have validated that login works. Logout with the below command.
```
$ make logout-user \
  COOKIE=<TOKEN FROM LOGIN COMMAND>
```
