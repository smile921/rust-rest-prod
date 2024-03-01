
touch .env
echo '' >.env

cargo install sea-orm-cli
sea-orm-cli migrate init
shelter_main.exe createadmin -p Password$$$123s


git config user.name "Smile921"  
git config user.email "smile921@smile921.org"