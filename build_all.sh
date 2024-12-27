#!/bin/zsh

print -n "Building server..."
pushd intranet_server > /dev/null || { echo "Server directory not found!"; exit 1; }
cargo build -r > /dev/null 2>&1
popd > /dev/null
echo "✓"

print -n "Building site..."
pushd intranet-website > /dev/null || { echo "Directory not found!"; exit 1; }
ng build --configuration production > /dev/null 2>&1
popd > /dev/null
echo "✓"

print -n "Moving files to static...."
eval cp -r intranet-website/dist/intranet-website/browser/* intranet_server/static
echo "✓"
