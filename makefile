.PHONY: pre-commit commit push confirm-prod push-prod

pre-commit:
	cargo fmt && cargo test && cargo doc

commit: pre-commit
	git commit

push:
	git push GitHub dev

confirm-prod:
	scripts/confirm-prod.sh

push-prod: confirm-prod
	echo "Pushed to prod"
