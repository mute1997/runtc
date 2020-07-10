IMAGE=runtime-tools
LOG=resources/runtc.log

all:
	touch ${LOG}
	truncate ${LOG} --size 0
	cargo build
	make validation || cat ${LOG}

validation:
	docker run \
		--rm \
		-it \
		-w /runtime-tools \
		-v ${PWD}:/runtc\
		${IMAGE} make RUNTIME=runtc localvalidation

docker:
	cd resources && docker build . -t ${IMAGE}
