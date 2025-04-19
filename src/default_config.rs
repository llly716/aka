pub const CLASH: &str = r#"mode: rule
ipv6: true
allow-lan: false
log-level: info
mixed-port: 12345
unified-delay: false
tcp-concurrent: true
external-controller: 127.0.0.1:9090
find-process-mode: strict
global-client-fingerprint: firefox
geodata-mode: true
geo-auto-update: true
geo-update-interval: 24
geodata-loader: memconservative
geox-url:
  geoip: "https://mirror.ghproxy.com/https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geoip.dat"
  geosite: "https://mirror.ghproxy.com/https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geosite.dat"
  mmdb: "https://mirror.ghproxy.com/https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/country.mmdb"
  asn: "https://mirror.ghproxy.com/https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/GeoLite2-ASN.mmdb"

profile:
  store-selected: true
  store-fake-ip: true

hosts:
  time.facebook.com: 17.253.84.125
  time.android.com: 17.253.84.125

sniffer:
  enable: true
  sniff:
    HTTP:
      ports: [80, 8080-8880]
      override-destination: true
    TLS:
      ports: [443, 8443]
    QUIC:
      ports: [443, 8443]
  skip-domain:
    - "Mijia Cloud"
    - "+.push.apple.com"

dns:
  enable: true
  ipv6: true
  prefer-h3: true
  respect-rules: true
  enhanced-mode: fake-ip
  fake-ip-filter:
    - "*"
    - "+.lan"
    - "+.local"
    - "+.market.xiaomi.com"
  default-nameserver:
    - https://1.1.1.1/dns-query#h3=true
    - https://1.0.0.1/dns-query#h3=true
    - quic://223.5.5.5
    - quic://223.6.6.6
  nameserver:
    - https://1dot1dot1dot1.cloudflare-dns.com/dns-query#h3=true
    - https://1.1.1.1/dns-query#h3=true
    - https://1.0.0.1/dns-query#h3=true
  proxy-server-nameserver:
    - quic://dns.alidns.com
    - quic://223.5.5.5
    - quic://223.6.6.6
  nameserver-policy:
    "geosite:cn,private":
      - quic://dns.alidns.com
      - quic://223.5.5.5
      - quic://223.6.6.6
    "geosite:geolocation-!cn":
      - https://1dot1dot1dot1.cloudflare-dns.com/dns-query#h3=true
      - https://1.1.1.1/dns-query#h3=true
      - https://1.0.0.1/dns-query#h3=true

tun:
  enable: true
  stack: system
  device: utun-akasha
  gso: true
  dns-hijack:
    - "any:53"
    - "tcp://any:53"
  auto-route: true
  auto-redirect: true
  auto-detect-interface: true

proxy-groups:
  - name: 世界树枝干
    type: select
    interval: 0
    url: http://cp.cloudflare.com
    proxies:
      - DIRECT
      - 选择节点
      - 自动回退
  - name: 选择节点
    type: select
    interval: 0
    url: http://cp.cloudflare.com
    include-all: true
  - name: 自动回退
    type: fallback
    interval: 0
    url: http://cp.cloudflare.com
    include-all: true
  - name: 国内
    type: select
    interval: 0
    proxies:
      - DIRECT
      - 世界树枝干
  - name: 广告
    type: select
    interval: 0
    url: http://cp.cloudflare.com
    proxies:
      - REJECT
      - REJECT-DROP
      - DIRECT
      - 世界树枝干
  - name: 其他
    type: select
    interval: 0
    url: http://cp.cloudflare.com
    proxies:
      - 世界树枝干
      - DIRECT
rules:
  - IP-CIDR,224.0.0.0/3,DIRECT
  - IP-CIDR6,ff00::/8,DIRECT
  - GEOIP,private,DIRECT
  - DOMAIN,www.nahida.im,DIRECT
  - GEOIP,CN,国内
  - GEOSITE,CN,国内
  - GEOSITE,category-ads-all,广告
  - MATCH,其他"#;
