# `autocuber`

To send HTML/JS/WASM files to the Thirds server (assuming you have an SSH connection), run these commands in a sh-like terminal:
```sh
cd www
rm -r dist
npm run build
rsync -avrz --delete --rsh=ssh ./dist/ root@thirdsgames.co.uk:/home/autocuber
```
