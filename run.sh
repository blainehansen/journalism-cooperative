to_dir=$1
shift
echo $to_dir
echo "$@"

pushd $to_dir
"$@"
popd
