include .env
export
.PHONY: build deploy generate-abis verify-keystore account chain compile deploy-verify flatten fork format lint test verify

chain:
	scripts/nitro-testnode.sh --init 
# List account
accounts:
	./nitro-testnode/test-node.bash script print-address --account sequencer

# Deploy the project
deploy:
	@if [ '$(RPC_URL)' = 'localhost' ]; then \
		cargo stylus deploy --endpoint="http://localhost:8547" --private-key="${PRIVATE_KEY}" --estimate-gas; \
	else \
		cargo stylus deploy --endpoint="${RPC_URL}" --private-key="${PRIVATE_KEY}" --estimate-gas; \
	fi

# Build and deploy target
build-and-deploy: build deploy generate-abis

# Generate TypeScript ABIs
generate-abis:
	mkdir -p abi
	touch abi/Interfaces.sol
	cargo stylus export-abi > abi

# List account

# Generate a new account
account-generate:
	@cast wallet import $(ACCOUNT_NAME) --private-key $$(cast wallet new | grep 'Private key:' | awk '{print $$3}')
	@echo "Please update .env file with ETH_KEYSTORE_ACCOUNT=$(ACCOUNT_NAME)"

# Compile contracts
compile:
	cargo stylus check 
