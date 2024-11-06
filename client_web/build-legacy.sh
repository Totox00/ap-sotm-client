rm -rf legacy
rm -rf www/legacy
mkdir legacy
mkdir www/legacy
cd legacy
git clone https://github.com/Totox00/ap-sotm-client.git
cd ap-sotm-client
git checkout legacy
cd client_web
./build.sh
cd ../../../
cp -r legacy/ap-sotm-client/client_web/www/pkg www/legacy
cp legacy/ap-sotm-client/client_web/www/index.html www/legacy
cp legacy/ap-sotm-client/client_web/www/common.css www/legacy
cp legacy/ap-sotm-client/client_web/www/index.js www/legacy
