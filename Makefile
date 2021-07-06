IMG ?= fade2019/metaverse:v0.0.1

all: clean ci cd

local_debug_run:
	cargo run --release -- --validator --tmp --rpc-cors -ldebug all

ci: build-target push

build-target:
	docker build -t $(IMG) .

push:
	docker push $(IMG)

cd:
	kubectl config use-context local
	kubectl apply -f ./k8s/boot-node
	kubectl apply -f ./k8s/validator

del:
	kubectl config use-context local
	kubectl delete -f ./k8s/boot-node
	kubectl delete -f ./k8s/validator

clean:
	@rm -rf ./target/*
