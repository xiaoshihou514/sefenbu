![版本](https://img.shields.io/crates/v/sefenbu)

通过可视化一张图片的颜色分布来帮助你根据图片设计配色。支持OKHSV，OKHSL，HSV，HSL色彩空间。

|                                `sefenbu -u okhsv test.jpg`                                |                                `sefenbu -u okhsl test.jpg`                                |
| :---------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------: |
| ![okhsv](https://github.com/user-attachments/assets/b480a782-d129-4d35-9ee6-2712e9b2cf8d) | ![okhsl](https://github.com/user-attachments/assets/3ee76175-b631-4162-bc9f-8121644ade14) |
|                                 `sefenbu -u hsv test.jpg`                                 |                                 `sefenbu -u hsl test.jpg`                                 |
|  ![hsv](https://github.com/user-attachments/assets/f30a24dd-dcbc-4c94-b64f-df51226bf179)  |  ![hsl](https://github.com/user-attachments/assets/6d682129-6cf7-488d-9923-e31ead506800)  |

## 用法

```
Usage: sefenbu [OPTIONS] <FILE>

Arguments:
  <FILE>  Input image

Options:
  -u, --using <USING>  Color space
  -h, --help           Print help
  -V, --version        Print version
```

你还需要将`shaders/`文件夹放在当前目录以供加载。

## 安装

系统依赖为[bevy依赖](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies)。

```shell
cargo install sefenbu
```

如果你用X11，把`Cargo.toml`里的wayland flag去掉以编译。

## 鸣谢

- 感谢[TD-Sky](https://github.com/TD-Sky)义务帮我解决rust的各种问题！
- 感谢[char-BS](https://github.com/char-BS)包涵我爱做项目的瘾！
