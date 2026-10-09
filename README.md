# BnetSwitchLite

本仓库基于原作者 [kabxx/BnetSwitchLite](https://github.com/kabxx/BnetSwitchLite) 修改，感谢原作者提供的 Battle.net 账号切换工具。

## 与原版的区别

- **一次登录即可保存**：原流程需要先在战网登录、刷新账号，再点击“登录并保存”重新登录。本版新增“添加账号”，在战网登录一次后自动识别并保存登录状态。
- **首次使用可直接添加**：无需预先生成账号记录或战网配置文件，未设置客户端路径时会打开选择器。
- **支持取消添加**：取消登录会恢复登录前的战网配置；保存前仍会校验认证状态和账号身份。
- **调整底部工具栏**：增加添加入口，适配窄窗口。

## 下载与编译

本仓库的 [Releases](https://github.com/PAKingdom/BnetSwitchLite/releases) 提供 Windows x64 便携 EXE，无需安装。

其他平台或架构版本请自行编译，构建说明见 [开发者文档](docs/DEVELOPMENT.md)。

## 贡献

欢迎向本仓库 [PAKingdom/BnetSwitchLite](https://github.com/PAKingdom/BnetSwitchLite) 提交 PR。

## 许可证

沿用原项目的 [MIT License](LICENSE)。第三方依赖说明见 [NOTICE](NOTICE)。
