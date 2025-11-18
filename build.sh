# TIME START
start=$(date +%s%N)

echo "\033[0;35m🔨 BUILD\033[0m → building frontend\033[0m"
cd frontend
yarn
yarn build

echo "\033[0;35m🔨 BUILD\033[0m → building backend\033[0m"
cd ../backend
cargo build --release

# TIME END
end=$(date +%s%N)

echo "\033[0;35m🔨 BUILD\033[0m completed in \033[0;32m$(($(($end-$start))/1000000)) ms\033[0m"