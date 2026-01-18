# Arch Linux Install

下面的步骤是基于 UEFI 模式的安装, 其它方式不能盲目执行

## 连接网络

```bash
# 查看电脑上的网卡列表, 一般我们 wifi 连接的都是 wlan0 这一块网卡
ip a

# 无线网络连接工具, 命令进入无线网络配置工作区内
# iwctl 是 iNet 无线守护程序 (iwd) 的命令行客户端工具，主要用于在 Arch Linux 安装环境或超轻量级系统中连接 Wi-Fi
# 它于 2020 年取代了 wifi-menu，成为 Arch ISO 官方首选的无线连接工具
iwctl

# 列出所有的网络设备信息列表, 通常是 ip a 里面看到的 wlan0 是有效的
device list

# 扫描网络, 通常看不到任何输出, 执行完毕即可
station wlan0 sacn

# 列出所有可用的网络连接, 通常就能看到自己常用的 wifi 名称
station wlan0 get-networks

# 连接我们的 wifi 名称, 比如 TP-Link_xxx, 然后输入密码建立连接, 此时的命令行界面是没法输入中文等特殊字符的, 所以 Wifi 名称不能是非英语世界的字符, 否则可能无法输入
station wlan0 connect TP-Link_xxx

# 退出 iwctl 命令界面
exit

# 测试网络是否连通, 有收到回应包即可
ping www.google.com

# 同步 UTC 时间, 显示 NTP service 是否为 active 即可
timedatectl

# 如果没有同步 NTP service , 手动执行同步, 不配置后续的安装可能无法通过软件安全验证
timedatectl set-ntp true
```

## 配置镜像源

```bash
# reflector 自动配置镜像源工具
# reflector 是 Arch Linux 中一个非常实用的 Python 脚本，专门用于从 Arch Linux Mirror Status 页面获取最新的镜像列表，并根据速度、稳定性和地理位置进行筛选，自动更新 /etc/pacman.d/mirrorlist 文件
# -a 最近12小时更新过的源
# -c 指定所在的国家或地区 cn 是中国
# -f 挑选最快的10个
# --sort score 按照下载时间和同步速度综合评分排序
# --v 把这个过程显示出来
reflector -a 12 -c cn -f 10 --sort score --v --save /etc/pacman.d/mirrorlist

# 更新数据库并安装密钥, 回车确认安装即可
pacman -Sy archlinux-keyring
```

## 磁盘分区

```bash
# 查看当前分区情况, -p 列出完整设备名 f 查看详细信息, 找到自己的磁盘, 一般是类似 /dev/nvme0n1 这样子的
lsblk -pf

# 列出该块硬盘更详细的信息确认是否要安装系统的磁盘, 仔细看不要选错
fdisk -l /dev/nvme1n1

# 进入磁盘分区界面, 硬盘是第一次使用会弹出分区类型, 一般选择 GPT 即可
cfdisk /dev/nvme1n1

# 创建一个启动分区, 100M 即可, 类型选择 EFI System, 根据界面进行操作即可
# esp 分区的常用挂载点一般有三个
# /boot 最常用的是 /boot, 通常存放内核和系统化相关的文件, 而且内核文件体积大, 如果放在该目录则需要更大的分区, 通常是 1-2 G 左右, 否则安装多个内核可能磁盘空间不足, 但是现在很多发行版都不将内核放入该目录了, 而是放在了根分区, 这里仅存启动相关的文件, 该空间现在就变得很小了, 通常 100M 足够了
# /boot/efi
# /efi

# 创建根分区
# 根分区的文件系统
# ext4 稳定可靠
# btrfs 但是对于 arch 滚动发行来说, btrfs 是更好的选择, 能更好的支持快照
# ESP 文件系统必须是 FAT 格式, 所以无法被 btrfs 快照

# 查看刚才的分区结果, 确认盘符
lsblk -pf

# 格式化分区, 把 ESP 分区格式化为 FAT32
mkfs.fat -F 32 /dev/nvme1n1p1

# 把根分区格式化为 btrfs
mkfs.btrfs /dev/nvme1n1p2
```

## 创建子卷和挂载分区

```bash
# 创建子卷, 作用是确定 btrfs 快照的范围, 如果不创建快照就会把系统数据和用户数据一起存档, 恢复时会把用户数据也给回档

# 把根分区挂载到 /mnt 上 指定文件系统是 btrfs, 接下来 /mnt 就是我们要安装系统的根
mount -t btrfs /dev/nvme1n1p2 /mnt

# 创建 root 子卷, 子卷的名称是 @
btrfs subvolume create /mnt/@

# 创建 home 子卷
btrfs subvolume create /mnt/@home

# 关于交换空间 swap, 目的是存储内存中的冷数据, 还能在内存不够用的时候把硬盘当作虚拟内存使用等, 但是这些对桌面用户意义不大

# 取消挂载, 重新挂载 子卷 到 /mnt
umount /mnt

# 挂载 root 子卷到 /mnt -o 指定挂载参数, 参数之间用逗号分隔
mount -t btrfs -o subvol=/@,compress=zstd /dev/nvme1n1p2 /mnt

# 挂载 home 子卷到 /mnt/home --mkdir 创建 home 目录
mount --mkdir -t btrfs -o subvol=/@home,compress=zstd /dev/nvme1n1p2 /mnt/home

# 挂载 esp, 常用的 /boot 不能用, 挂载到 /efi 扁平布局更整洁, arch wiki 也推荐挂载到 /efi
mount --mkdir /dev/nvme1n1p1 /mnt/efi
```

## 安装系统

```bash
# 安装基本系统
# pacstrap 把指定的软件安装到根目录
# pacstrap 是 Arch Linux 安装过程中的核心工具，它是一个 Bash 脚本，专门用于将基础软件包安装到指定的挂载目录下（通常是 /mnt），从而构建出一个能够自主运行的 Linux 系统根目录
# -K 复制密钥 初始化新系统的 pacman 信任链，将主机的 GPG 密钥复制到新系统，避免安装后出现签名验证问题
# /mnt 我们要安装系统的根
# base 基本包
# base-devel 是后续编译需要安装的工具包
# linux-zen 主线内核, -zen 是性能特调内核
# linux-firmware 基本固件
# btrfs-progs btrfs 管理工具
pacstrap -K /mnt base base-devel linux-zen linux-firmware btrfs-progs

# 安装其它包
# networkmanager 联网工具, 主流桌面环境都使用该工具, 一定要安装否则后续进入系统后无法联网, iwctl 命令行方式联网也可以
# intel-ucode 优化和修复 cpu, 如果是 amd 改为 amd-ncode
pacstrap /mnt networkmanager vim sudo intel-ucode

# 生成 fstab 文件, 系统启动时会自动安装里面的内容进行挂载
genfstab -U /mnt > /mnt/etc/fstab

# 进入新安装的系统
arch-chroot /mnt
```

## 初期配置

```bash
# 设置时区 /etc/localtime 这类文件一开始都是不存在的执行完毕后才会生成
ln -s /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

# 也可以直接设置时区
timedatectl set-timezone Asia/Shanghai

# 检查时区是否设置正确, 如果 Time zone 显示的不是 Asia/Shanghai 则需要手动设置才生效
timedatectl

# 生成 /etc/adjtime 文件, 系统用来调整时间误差
hwclock --systohc

# 系统本地化设置 
# 开启 en_US.UTF-8 UTF-8
# 开启 zh_CN.UTF-8 UTF-8
vim /etc/locale.gen

# 生成本地化文件
locale-gen

# 设置本地化
# 设置全局本地化 LANG=en_US.UTF-8
vim /etc/locale.conf

# 设置主机名, 自定义即可, 比如我的是主板 b760m 小写即可
vim /etc/hostname

# 设置 root 账户的密码
passwd

# 安装引导加载程序
# 一般来说 grub 功能强大兼容性好
# efibootmgr 管理 efi 启动项
pacman -S grub efibootmgr

# 然后用 grub 安装引导
# --target 指定架构是 x86_64 efi 固件
# --efi-directory efi 安装目录
# --boot-directory boot 安装目录, 如果不指定默认会安装到 /boot 中 /boot 是 btrfs 格式, 初期会无法写入一些文件会出问题
# --bootloader-id 取一个喜欢的启动项名称, 不指定默认是 ArchLinux
grub-install --target=x86_64-efi --efi-directory=/efi --boot-directory=/efi

# 大部分软件会认为 grub 安装在 /boot 目录下, 但是我们明确安装在 /efi 下, 为了避免找不到, 我们将它连接到 /efi
ln -s /efi/grub /boot/grub

# 生成 grub 配置文件, 会生成具体的启动项目和流程
grub-mkconfig -o /efi/grub/grub.cfg

# 编辑生成的默认 grub 文件, 把日志等级改为 5
# GRUB_CMDLINE_LINUX_DEFAULT="loglevel=5" 删除 quiet 等其它字符即可
vim /etc/default/grub

# 重新生成 grub.cfg 配置文件
grub-mkconfig -o /efi/grub/grub.cfg

# 因为我们没有配置 swap 交换分区, 所以我们配置内存压缩 zram
pacman -S zram-generator

# 编辑 zram 配置文件启用并配置大小
# [zram0]
# zram-size = ram
# compression-algorithm = zstd
#
# zram-size 最多使用多少数据, 压缩前的内容桌面端跟内存大小一致即可
# compression-algorithm 使用 zstd 压缩算法
vim /etc/systemd/zram-generator.conf

# 再次编辑 grub 默认配置文件
# 在刚才的日志配置中继续追加 zswap.enabled=0 禁用 zswap 因为我们根本没有设置 zswap 交换空间
vim /etc/default/grub

# 重新生成启动文件
grub-mkconfig -o /efi/grub/grub.cfg

# 完成所有的配置 退出chroot环境
exit

# 重启进入我们刚配置的系统
reboot
```

## 配置系统

```bash
# 重启后使用 root 账户登录

# 开启网络服务
systemctl enable --now NetworkManager

# 打开网络连接界面, 配置网络即可
nmtui

# 更新系统
pacman -Syu

# 重新进入系统
reboot

# 创建用户并加入 wheel 组, 该组已经具有 sudo 权限 设置密码
useradd -mG wheel zhangguangxun
passwd zhangguangxun

# 开启 wheel 权限组
# 取消该行注释
# %wheel ALL=(ALL:ALL) ALL NOPASSWD: ALL
visudo

# 开启 32 位源, 对于桌面端用户来说是必须的
# 取消下面的注释
# [multilib]
# Include = /etc/pacman.d/mirrorlist
vim /ect/pacman.conf

# 安装快照需要的包
# snapper 自动化管理快照的命令行工具
# snap-pac snap-pac 执行 pacman 操作时自动创建快照
# btrfs-assistant 图形化交互工具
# grub-btrfs 自动在启动菜单里添加快照启动项
# inotify-tools
pacman -S snapper snap-pac btrfs-assistant grub-btrfs inotify-tools

# 重启让自动创建快照功能生效
reboot

# 创建快照, 设置快照的范围是根目录
snapper -c root create-config /

# 创建 home 配置, 可以不快照, 下一次重装系统时记得 单独分区 /home 后续重装就可以不用覆盖该目录能保存个人文件
snapper -c home ceate-config /home

# 开启快照
systemctl enable --now grub-btrfsd

# 开启自动清理机制
# 如果不配置，快照会占满你的 1T 硬盘。
# 修改配置： sudo nano /etc/snapper/configs/root
# 建议修改参数：
# TIMELINE_LIMIT_HOURLY="5"
# TIMELINE_LIMIT_DAILY="7"
# TIMELINE_LIMIT_WEEKLY="0"
# TIMELINE_LIMIT_MONTHLY="0"
# 这样只会保留最近几小时和最近几天的快照。
# 开启服务：

sudo systemctl enable --now snapper-timeline.timer
sudo systemctl enable --now snapper-cleanup.timer

# linux-lts 内核不会长期更新, 系统出现问题可以借助它进入系统排查是否是内核问题导致的系统错误, 我发觉这个可以不安装
# 如果是可以用 downgrade 进行系统降级, downgrade 需要从 aur 安装
# aur 不稳定, 最起码我的电脑就不稳定, 所以不安装
# pacman -S linux-lts

# 快照使用, 列出所有的快照版本
# snapper -c root list
# 使用 undochange 回档, 但是不建议使用 undochange 回档 root 目录, 会导致未知的问题
# snapper -c root undochange 数字是你的快照版本号
# 所以推荐是从快照启动项进入系统选择快照版本来回档

# 然后使用 btrfs-assistant 助手查看快照列表
# btrfs-assistant -l

# 用小写的 -r 进行恢复
# btrfs-assistant -r 到某个版本

# 处理快照最好以 root 登录操作
```

## 显卡驱动和视频编解码

```bash
# nvidia
# --needed 安装过就不再重新安装
# linux-zen-headers 编译需要依赖的头文件
# linux-lts-headers lts 内核可以不安装
sudo pacman -S --needed linux-zen-headers

# 可能需要借助 arch wiki 查看需要安装说明
# lib32-nvidia-utils 一般会自动安装, 如果没有可以手动安装
sudo pacman -S nvidia-open-dkms nvidia-utils lib32-nvidia-utils

# 安装视频编解码, amd 无需安装默认已提供
sudo pacman -S intel-media-driver

# nvidia 也可以支持, 一般也安装
sudo pacman -S --needed libva-nvidia-driver

# 重启
sudo reboot

# 安装声卡固件
# 安装 ALSA（高级 Linux 声音架构）底层的固件和配置脚本
# sof-firmware 主要针对 Intel 酷睿系列（Skylake 及更新） 以及部分 AMD/联发科平台的笔记本电脑提供开源的音频数字信号处理（DSP）固件
# alsa-ucm-conf 提供硬件“用例”配置脚本
# alsa-firmware 不常见或者比较旧的设备提供固件
sudo pacman -S sof-firmware alsa-ucm-conf alsa-firmware

# pipewire 红帽主导开发的新兴音视频服务技术
# wireplumber 会话管理器
# pipewire-pluse 建立一个模拟 PulseAudio 的服务端 绝大多数桌面应用（如 Chrome、Firefox、Discord、网易云音乐）都使用 PulseAudio 协议 安装它后，这些应用会认为系统仍在运行 PulseAudio，从而无缝工作
# pipewire-alsa 为那些直接调用 ALSA 低层接口的旧软件提供兼容
# pipewire-jack 为专业音频软件（如 Ardor, Bitwig, OBS 的某些插件）提供支持 它允许专业音频应用以极低延迟运行 且能与普通桌面应用同时发声
# 安装完成后，PipeWire 通常由用户级别的 systemd 自动管理 无需手动 sudo systemctl enable（在图形界面登录时会自动拉起）
sudo pacman -S pipewire wireplumber pipewire-pulse pipewire-alsa pipewire-jack

# 安装蓝牙
sudo pacman -S bluez

# 开启蓝牙系统服务
sudo systemctl enable --now bluetooth

# 电源切换工具 power-profiles-daemon 各个桌面通用的可以安装

# 安装中文字体
# noto-fonts 谷歌开源字体
# adobe-source-han-sans-cn-fonts 中文思源字体
sudo pacman -S noto-fonts noto-fonts-emoji adobe-source-han-sans-cn-fonts

# 重启系统
reboot

# 安装桌面环境
sudo pacman -S gnome

# 配置启动图形界面
sudo systemctl enable --now gdm.service

# 到此系统基本就安装完毕了
```

## 输入法

目前中文输入法选择中州韵即可

官方文档给的安装方法不完整, 目前推荐使用 fctix5-rime

fcitx5 主输入法框架, 一般会被依赖安装

fcitx5-gtk, fcitx5-qt 针对最受欢迎的几种UI开发工具包的输入法模块, 因为你的图形界面可能有的是 qt 开发的, 有的是 gtk 开发的所以一般都需要安装

fcitx5-configtool GUI配置程序, 需要安装

fcitx5-chinese-addons 中文支持

实际上我看我安装的包如下

```bash
[zhangguangxun@b760m ~]$ pacman -Qs fcitx5
local/fcitx5 5.1.17-1 (fcitx5-im)
    Next generation of fcitx
local/fcitx5-configtool 5.1.12-1 (fcitx5-im)
    Configuration Tool for Fcitx5
local/fcitx5-qt 5.1.11-3 (fcitx5-im)
    Fcitx5 Qt Library (Qt5 & Qt6 integrations)
local/fcitx5-rime 5.1.12-1
    RIME support for Fcitx5
[zhangguangxun@b760m ~]$ 
```

所以应该安装下面两个包就够用了

```bash
sudo pacman -S fcitx5-im fcitx5-rime
```

使用 Rime 时默认可以按 F4 或 Ctrl+` 切换输入法

高级配置就看自己需要即可