cd web/client

echo "------------------------"
echo "Installing client's dependencies..."
echo "------------------------"

npm i

cd ../..

echo "------------------------"
echo "Install wasm-pack ?"
echo "------------------------"
select yn in "Yes" "No"; do
    case $yn in
        Yes ) curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; break;;
        No ) exit;;
    esac
done

echo "------------------------"
echo "Install watchexec-cli ?"
echo "------------------------"
select yn in "Yes" "No"; do
    case $yn in
        Yes ) cargo install watchexec-cli; break;;
        No ) exit;;
    esac
done

echo "------------------------"
echo "Install async-cmd ?"
echo "------------------------"
select yn in "Yes" "No"; do
    case $yn in
        Yes ) cargo install async-cmd; break;;
        No ) exit;;
    esac
done

echo "------------------------"
echo "Compiling WASM"
echo "------------------------"
cd zzg_wasm
wasm-pack build --out-dir ../web/client/src/zzg