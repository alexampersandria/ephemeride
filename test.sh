# TIME START
start=$(date +%s%N)

echo "\033[0;35m👩‍🔬 TEST\033[0m → backend\033[0m"
cd backend
cargo test

echo "\033[0;35m👩‍🔬 TEST\033[0m → frontend\033[0m"
cd ../frontend
yarn test

# TIME END
end=$(date +%s%N)

echo "\033[0;35m👩‍🔬 TEST\033[0m completed in \033[0;32m$(($(($end-$start))/1000000)) ms\033[0m"