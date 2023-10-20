IMAGE=offchain-metadata-api
# AWS_REGION=eu-central-1
# ECR_REPOSITORY=XXX.dkr.ecr.eu-central-1.amazonaws.com

login:
	aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin ${ECR_REPOSITORY}

build:
	docker build -t ${IMAGE} .
	docker tag ${IMAGE}:latest ${ECR_REPOSITORY}/${IMAGE}:latest

push:
	docker push ${ECR_REPOSITORY}/${IMAGE}:latest

develop:
	RUST_LOG=debug cargo run

dockerdevelop:
	docker run -it --rm --name ${IMAGE} \
		-v $(pwd)/registry_data:/registry \
		-p 8080:8080 \
		-e RUST_LOG=debug \
		${IMAGE}:latest
