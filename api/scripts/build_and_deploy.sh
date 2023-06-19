#! /bin/bash
set -e

printf "Running build... \n\n"
cargo build --release

printf "\n\nDeploying changes...\n\n"
rsync -avz --progress --delete ./target/release/vignette root@kiri.ai:/usr/local/bin/vignette-api
rsync -avz --progress --delete ./scripts/vignette-api.service root@kiri.ai:/etc/systemd/system/vignette-api.service
rsync -avz --progress --delete ./scripts/default_data/* root@kiri.ai:/var/tmp/vignette/default
ssh root@kiri.ai 'systemctl daemon-reload; systemctl restart vignette-api.service; systemctl status vignette-api.service'

printf "\n\nServer Deployment complete!"
