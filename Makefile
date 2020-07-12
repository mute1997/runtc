IMAGE=runtime-tools
LOG=resources/runtc.log
# CMD=/bin/bash
CMD=make RUNTIME=runtc localvalidation
# CMD=make RUNTIME=runtc localvalidation VALIDATION_TESTS=./validation/linux_cgroups_devices/linux_cgroups_devices.t
# CMD=make RUNTIME=runtc localvalidation VALIDATION_TESTS=./validation/linux_cgroups_cpus/linux_cgroups_cpus.t
# CMD=make RUNTIME=runtc localvalidation VALIDATION_TESTS=./validation/linux_cgroups_memory/linux_cgroups_memory.t
# CMD=make RUNTIME=runtc localvalidation VALIDATION_TESTS=./validation/linux_cgroups_hugetlb/linux_cgroups_hugetlb.t

# XXX
# これを通るようにする
# runcも使って, 通るかテストしてみる
# ./validation/linux_cgroups_cpus/linux_cgroups_cpus.t
# ./validation/linux_cgroups_memory/linux_cgroups_memory.t

# TODO
# open /sys/fs/cgroup/blkio/cgrouptest/blkio.weight: permission denied
# いくつかDocker環境なので動いてない機能があるっぽい
# 仮想環境に同じ環境を立ち上げてテストしてみる (KVMでやってみる)
# 仮想環境でも変わらないようであればDockerでテストを続ける

all:
	touch ${LOG}
	truncate ${LOG} --size 0
	cargo build
	docker run \
		--privileged \
		--rm \
		-it \
		-w /runtime-tools \
		-v ${PWD}:/runtc\
		${IMAGE} ${CMD}

test:
	cargo test

docker:
	cd resources && docker build . -t ${IMAGE}
