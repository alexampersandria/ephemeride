echo "\033[0;35m⚙️ VERSION\033[0m \033[0;33m⚠️ warning: versioning is unstable\033[0m"

version="$(cat VERSION)"
echo "\033[0;35m⚙️ VERSION\033[0m → versioning: \033[0;32m${version}\033[0m"

# set version in frontend/package.json and backend/Cargo.toml
echo "\033[0;35m⚙️ VERSION\033[0m → updating frontend/package.json\033[0m"
sed -i.bak "s/\"version\": \".*\"/\"version\": \"${version}\"/" frontend/package.json
echo "\033[0;35m⚙️ VERSION\033[0m → updating backend/Cargo.toml\033[0m"
sed -i.bak "s/^version = \".*\"/version = \"${version}\"/" backend/Cargo.toml
echo "\033[0;35m⚙️ VERSION\033[0m versioning complete\033[0m"
