#
# Construction de la version de production
# qui sera installée dans le container docker silex
#

cargo build --release
cp target/release/silex /volshare/docker/silex/silex
cp .env /volshare/docker/silex/.env