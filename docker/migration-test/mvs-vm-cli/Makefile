UNAME_S := $(shell uname -s)
build: 
	go build ./cmd/mvs-vm-cli

install: build
	sudo cp mvs-vm-cli /usr/local/bin/

clean: 
	rm mvs-vm-cli

.PHONY: install test build
