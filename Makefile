######################
# DEVELOPMENT COMMANDS
######################
local: local-db watch 

clean-local: clean-db

timeout:
	sleep 5

watch:
	cargo watch -x run

###################
# DATABASE COMMANDS
###################
.PHONY: db local-db clean-db
db:
	sudo docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_USER=admin -p 5432:5432 -d postgres
	sudo docker ps

local-db: db timeout orm-setup orm-migration

clean-db:
	sudo docker kill postgres
	sudo docker rm postgres

##############
# ORM COMMANDS
##############
.PHONY: table orm-migration orm-setup
# make table NAME="<new table name>
table:
	diesel migration generate $(NAME)

orm-migration:
	diesel migration run

orm-setup:
	diesel setup

##################
# FUNCTIONAL TESTS
##################
# note: This assumes you have everything running locally
.PHONY: invite-user register-user login-user logout-user

# `$ make invite-user EMAIL=""`
invite-user:
	curl -d '{"email": "$(EMAIL)"}' -H 'Content-Type: application/json' http://localhost:3000/api/invitation

# `$ make register-user INVITATION_ID="" EMAIL="" PASSWORD=""`
register-user:
	curl --request POST \
	 --url http://localhost:3000/api/register/$(INVITATION_ID) \
	 --header 'content-type: application/json' \
	 --data '{"email":"$(EMAIL)", "password":"$(PASSWORD)"}'

# `$ make login-user EMAIL="" PASSWORD=""
login-user:
	curl -i --request POST \
		--url http://localhost:3000/api/auth \
		--header 'content-type: application/json' \
	  --data '{"email":"$(EMAIL)", "password":"$(PASSWORD)"}'

# `$ make test-auth COOKIE=""
test-auth:
	curl -i --request GET \
		--url http://localhost:3000/api/auth \
		--cookie "auth=$(COOKIE)"

# `$ make logout-user COOKIE=""
logout-user:
	curl -i --request DELETE \
		--url http://localhost:3000/api/auth
	  --cookie "auth=$(COOKIE)"
