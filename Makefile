REGION=eu-central-1


# Some python helper
venv:
	: # Create venv if it doesn't exist
	: # test -d venv || virtualenv -p python3 --no-site-packages venv
	test -d .venv || python3 -m venv .venv

install: venv
	: # Activate venv and install somthing inside
	(\
		source .venv/bin/activate;\
		pip install --upgrade pip\
		pip install requests\
	)

.PHONY: app
app:
	: # Run the app
	(\
		source .venv/bin/activate;\

	)


login:
	aws ecr get-login-password --region ${REGION} | docker login --username AWS --password-stdin 643981526071.dkr.ecr.${REGION}.amazonaws.com

build:
	docker build -t offchain-metadata-api  .
tag:
	docker tag offchain-metadata-api:latest 643981526071.dkr.ecr.eu-central-1.amazonaws.com/offchain-metadata-api:latest

push:
	docker push 643981526071.dkr.ecr.eu-central-1.amazonaws.com/offchain-metadata-api:latest