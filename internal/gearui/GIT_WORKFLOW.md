# GearUI Git 工作流程

## 📋 仓库结构

```
upstream (slint-ui/slint)     ← Slint 官方仓库（只读）
    ↓
origin (你的用户名/slint)     ← 你的 Fork（可写）
    ↓
本地 gearui-dev 分支          ← 日常开发分支
```

## 🚀 初始设置（只需一次）

### 1. Fork Slint 仓库

1. 访问 https://github.com/slint-ui/slint
2. 点击 "Fork" 按钮
3. Fork 到你的 GitHub 账号

### 2. 配置 Git Remote

```bash
cd slint

# 将现有的 origin 重命名为 upstream
git remote rename origin upstream

# 添加你的 fork 为 origin（替换 YOUR_USERNAME）
git remote add origin git@github.com:YOUR_USERNAME/slint.git

# 验证
git remote -v
```

预期输出：
```
origin    git@github.com:YOUR_USERNAME/slint.git (fetch)
origin    git@github.com:YOUR_USERNAME/slint.git (push)
upstream  git@github.com:slint-ui/slint.git (fetch)
upstream  git@github.com:slint-ui/slint.git (push)
```

### 3. 创建开发分支

```bash
# 创建并切换到开发分支
git checkout -b gearui-dev

# 提交 GearUI 代码
git add internal/gearui/ examples/gearui/ Cargo.toml Cargo.lock
git commit -m "feat(gearui): 添加 GearUI 核心库和示例程序"

# 推送到你的 fork
git push -u origin gearui-dev
```

---

## 💻 日常开发工作流程

### 开发新功能

```bash
# 1. 确保在开发分支
git checkout gearui-dev

# 2. 开发代码
# ... 编辑文件 ...

# 3. 查看更改
git status
git diff

# 4. 提交更改
git add internal/gearui/src/interactive/line_edit.rs
git commit -m "feat(gearui): 实现 LineEdit 组件"

# 5. 推送到远程
git push
```

### 同步 Slint 上游更新

```bash
# 1. 切换到 master 分支
git checkout master

# 2. 拉取上游最新代码
git fetch upstream
git merge upstream/master

# 3. 推送到你的 fork
git push origin master

# 4. 切换回开发分支
git checkout gearui-dev

# 5. 合并 master 到开发分支
git merge master

# 6. 解决冲突（如果有）
# ... 手动编辑冲突文件 ...
git add .
git commit -m "merge: 合并上游更新"

# 7. 推送
git push
```

### 快速同步命令（别名）

添加到 `~/.gitconfig`:

```ini
[alias]
    sync-upstream = "!f() { \
        git checkout master && \
        git fetch upstream && \
        git merge upstream/master && \
        git push origin master && \
        git checkout gearui-dev && \
        git merge master; \
    }; f"
```

使用：
```bash
git sync-upstream
```

---

## 📊 分支管理

### 分支说明

| 分支 | 用途 | 更新方式 |
|------|------|---------|
| `master` | 跟踪 Slint 官方 | 从 upstream 拉取 |
| `gearui-dev` | GearUI 开发主分支 | 日常开发 |
| `feature/*` | 功能分支（可选） | 从 gearui-dev 创建 |

### 创建功能分支（可选）

对于大型功能，可以创建独立分支：

```bash
# 从 gearui-dev 创建功能分支
git checkout gearui-dev
git checkout -b feature/line-edit

# 开发完成后合并回 gearui-dev
git checkout gearui-dev
git merge feature/line-edit
git push

# 删除功能分支
git branch -d feature/line-edit
```

---

## 🔄 常见场景

### 场景 1：放弃本地更改

```bash
# 放弃所有未提交的更改
git checkout .
git clean -fd

# 放弃特定文件
git checkout -- path/to/file.rs
```

### 场景 2：修改最后一次提交

```bash
# 修改提交信息
git commit --amend -m "新的提交信息"

# 添加遗漏的文件到上次提交
git add forgotten_file.rs
git commit --amend --no-edit

# 推送（如果已经 push 过，需要强制）
git push --force-with-lease
```

### 场景 3：查看提交历史

```bash
# 图形化查看
git log --oneline --graph --all -20

# 查看某个文件的历史
git log --oneline internal/gearui/src/lib.rs

# 查看某次提交的详情
git show <commit-hash>
```

### 场景 4：暂存当前工作

```bash
# 暂存未提交的更改
git stash

# 查看暂存列表
git stash list

# 恢复暂存
git stash pop

# 删除暂存
git stash drop
```

---

## 🎯 提交规范

### Commit Message 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试
- `chore`: 构建/工具链

### 示例

```bash
git commit -m "feat(gearui): 实现 LineEdit 组件

- 添加文本输入功能
- 支持光标移动
- 实现文本选择
- 添加键盘事件处理

Closes #123
"
```

---

## 📦 .gitignore 配置

确保 `slint/.gitignore` 包含：

```gitignore
# Rust
target/
Cargo.lock  # 对于库项目通常不提交

# IDE
.vscode/
.idea/
*.swp
*.swo

# macOS
.DS_Store

# 临时文件
*.tmp
*.bak
```

---

## 🛠️ 常用命令速查

```bash
# 查看状态
git status

# 查看远程仓库
git remote -v

# 查看分支
git branch -a

# 切换分支
git checkout gearui-dev

# 拉取上游
git fetch upstream

# 查看差异
git diff
git diff --staged

# 查看日志
git log --oneline -10

# 撤销更改
git checkout -- file.rs

# 重置到某个提交
git reset --hard <commit-hash>
```

---

## 🚨 注意事项

### ⚠️ 不要直接在 master 分支开发

```bash
# ❌ 错误
git checkout master
# 在 master 上开发...

# ✅ 正确
git checkout gearui-dev
# 在开发分支上开发...
```

### ⚠️ 同步前先提交本地更改

```bash
# 同步前检查
git status

# 如果有未提交的更改
git add .
git commit -m "WIP: 保存当前进度"

# 或者暂存
git stash
```

### ⚠️ 强制推送要谨慎

```bash
# ❌ 危险：可能丢失远程提交
git push --force

# ✅ 更安全：如果远程有新提交会失败
git push --force-with-lease
```

---

## 📚 参考资源

- [Git 官方文档](https://git-scm.com/doc)
- [GitHub Fork 工作流程](https://docs.github.com/en/get-started/quickstart/fork-a-repo)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

**最后更新**: 2026-02-02  
**维护者**: GearUI Team
