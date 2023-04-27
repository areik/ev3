PARENT_DIR_NAME := $(shell basename $(dir $(realpath $(firstword $(MAKEFILE_LIST)))))

.PHONY: app
app:
	@cd .. && \
	make app=$(PARENT_DIR_NAME) CC="ccache arm-none-eabi-gcc"
