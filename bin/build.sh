
OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

rm -rf ${THIS_FOLDER}/../bin/${OS}
mkdir -p ${THIS_FOLDER}/../bin/${OS}

echo "rust"
cd ${THIS_FOLDER}/../rust && cargo build --release
cp ${THIS_FOLDER}/../rust/target/release/actix ${THIS_FOLDER}/../bin/${OS}/wfbench_actix && strip ${THIS_FOLDER}/../bin/${OS}/wfbench_actix
cp ${THIS_FOLDER}/../rust/target/release/warp ${THIS_FOLDER}/../bin/${OS}/wfbench_warp && strip ${THIS_FOLDER}/../bin/${OS}/wfbench_warp
cp ${THIS_FOLDER}/../rust/target/release/axum ${THIS_FOLDER}/../bin/${OS}/wfbench_axum && strip ${THIS_FOLDER}/../bin/${OS}/wfbench_axum

echo "go"
cd ${THIS_FOLDER}/../go/net_http  && go build
cp ${THIS_FOLDER}/../go/net_http/net_http ${THIS_FOLDER}/../bin/${OS}/wfbench_net_http