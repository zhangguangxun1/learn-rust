# pacman 常用命令

1. 更新系统

更新系统通常意味着同步数据库并升级所有已安装的包

```bash
sudo pacman -Syu
```

2. 安装

```bash
sudo pacman -S 包名
```

3. 官方库中搜索包

```bash
pacman -Ss 关键字
```

4. 安装本地包（.pkg.tar.zst 格式）

```bash
sudo pacman -U /路径/到/文件.pkg.tar.zst
```

5. 仅删除包（保留其依赖）

```bash
sudo pacman -R 包名
```

6. 删除包及其不再需要的依赖

```bash
sudo pacman -Rs 包名
```

7. 彻底删除包、依赖及配置文件

```bash
sudo pacman -Rns 包名
```

8. 查看已安装的包

```bash
pacman -Qs 关键字
```

9. 查看某个包的详细信息（未安装时查看远程信息）

```bash
pacman -Si 包名
```

10. 查看某个已安装包的详细信息

```bash
pacman -Qi 包名
```

11. 清理缓存（删除旧版本的包）

```bash
sudo pacman -Sc
```

12. 找出系统中无用的孤立包（作为依赖被安装，但现在没被任何包使用）

```bash
pacman -Qdt
```

13. 一键清理孤立包

```bash
sudo pacman -Rs $(pacman -Qdtq)
```