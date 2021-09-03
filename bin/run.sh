
OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

BENCH_RESULT_DIR=${THIS_FOLDER}/../reports
BENCH_RESULT_FILE=${BENCH_RESULT_DIR}/${OS}.md

kill_processes() {
    kill -9 `ps aux | grep wfbench | grep -v grep |awk -F' ' '{ print $2 }'`
}

run_service() {
	local casename=$1
	local exe=wfbench_${casename}
	${THIS_FOLDER}/${OS}/${exe} &
	echo "start - ${exe}"
}


run_bench() {
	local casename=$1
	local port=$2
	local method=$3

	echo "run bench - ${casename}"

	echo "### ${casename}" >> ${BENCH_RESULT_FILE}
	echo '```text' >> ${BENCH_RESULT_FILE}

	if [ "${OS}" = "Darwin" ]; then
		echo "wrk(darwin) ... "
	    wrk --latency -t4 -c128 -d10s http://127.0.0.1:${port}/${method} >> ${BENCH_RESULT_FILE}
    else
    	echo "wrk(linux) ... "
		wrk --latency -t8 -c500 -d10s http://127.0.0.1:${port}/${method} >> ${BENCH_RESULT_FILE}
    fi
	echo '```' >> ${BENCH_RESULT_FILE}
}


kill_processes

mkdir -p ${BENCH_RESULT_DIR}
rm ${BENCH_RESULT_FILE}


echo " ---- start services ----"
run_service warp     8081 user
run_service actix    8082 user
run_service axum     8083 user
run_service poem     8084 user
run_service net_http 8091 user

#

sleep 5

echo " ---- run benchmarks ---- "
echo "## BENCHMARK REPORT" >> ${BENCH_RESULT_FILE}

run_bench warp     8081 user
sleep 50

run_bench actix    8082 user
sleep 50

run_bench axum     8083 user
sleep 50

run_bench poem     8084 user
sleep 50

run_bench net_http 8091 user

kill_processes
