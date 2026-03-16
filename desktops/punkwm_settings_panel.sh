# !/bin/bash
echo '{"sender":"PUNK_WM_APP_Open","category":"Settings Panel","name":"main","data":{}}' | nc -U /tmp/$(echo $USER)_punk_dock.sock ;