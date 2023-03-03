cd ../target/release/
tar -czf kai.tar.gz kai
mv kai.tar.gz ../../brew/
cd ../../brew/
shasum -a 256 kai.tar.gz
