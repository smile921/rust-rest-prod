
touch .env
echo '' >.env

cargo install sea-orm-cli
sea-orm-cli migrate init
shelter_main.exe createadmin -p Passwordxxx123s


git config user.name "xxx"  
git config user.email "xxx@xxx.org"