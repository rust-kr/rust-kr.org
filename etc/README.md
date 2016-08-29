So `systemd` such production
========

### Install
```shell
sudo cp rust-kr.service /etc/systemd/system/
sudo systemctl daemon-reload

sudo systemctl enable rust-kr
sudo systemctl start rust-kr
```
### Uninstall
```shell
sudo systemctl stop rust-kr
sudo systemctl disable rust-kr

sudo rm /etc/systemd/system/rust-kr.service
```
