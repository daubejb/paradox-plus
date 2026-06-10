.PHONY: critique-plan

critique-plan:
	@echo "Running automated implementation plan critique..."
	cargo run -p critique
