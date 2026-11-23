配置 VLESS 代理

Vmess 已经提示可能被移除，推荐使用 VLESS Encryption

VLESS + REALITY 配置不成功, 就不关注了

[Warning] common/errors: The feature VMess (with no Forward Secrecy, etc.) is deprecated, not recommended for using and might be removed. Please migrate to VLESS Encryption as soon as possible.

## 安装 Xray-core

代码库 [Xray-core](https://github.com/XTLS/Xray-core)

安装脚本 [Xray-install](https://github.com/XTLS/Xray-install)

通常下面的命令即可安装

```bash
bash -c "$(curl -L https://github.com/XTLS/Xray-install/raw/main/install-release.sh)" @ install
```

## 加解密

```bash
UUID=$(xray uuid)
SERVER_IP=$(curl -s4 ifconfig.me)
ENC_OUT=$(xray vlessenc)
DEC_KEY=$(echo "$ENC_OUT" | grep '"decryption"' | head -n 1 | awk -F'"' '{print $4}' | tr -d '\r\n')
ENC_KEY=$(echo "$ENC_OUT" | grep '"encryption"' | head -n 1 | awk -F'"' '{print $4}' | tr -d '\r\n')
```

## 配置文件

端口一般来说如果 443 没有被占用的话使用默认值, 否则自行放行其它端口

```bash
cat << EOF > /usr/local/etc/xray/config.json
{
  "log": {
    "loglevel": "warning"
  },
  "inbounds": [
    {
      "port": 443,
      "protocol": "vless",
      "settings": {
        "clients": [
          {
            "id": "${UUID}"
          }
        ],
        "decryption": "${DEC_KEY}"
      },
      "streamSettings": {
        "network": "tcp"
      },
      "sniffing": {
        "enabled": true,
        "destOverride": [
          "http",
          "tls",
          "quic"
        ]
      }
    }
  ],
  "outbounds": [
    {
      "protocol": "freedom",
      "tag": "direct"
    }
  ]
}
EOF
```

## 测试配置文件

```bash
xray run -test -config /usr/local/etc/xray/config.json
Xray 26.11.9 (Xray, Penetrates Everything.) 676eaf6 (go1.27.3 linux/amd64)
A unified platform for anti-censorship.
2026/11/23 14:37:44.925430 [Info] infra/conf/serial: Reading config: &{Name:/usr/local/etc/xray/config.json Format:json}
Configuration OK.
```

## 重启

```bash
sudo systemctl restart xray
```

## 链接

```bash
echo ""
echo "================ VLESS 原生加密配置成功 ================"
echo "vless://${UUID}@${SERVER_IP}:443?encryption=${ENC_KEY}&type=tcp#Ubuntu-VLESS-Enc"
echo "========================================================"
echo ""
```
