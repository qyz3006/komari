//! 简体中文语言包（外挂语言文件）
//!
//! 通过 `tr(key)` 查询中英文本地化映射；未命中时返回 `key` 本身（英文原文），
//! 因此未翻译的项会自动保持英文，绝不 panic。
//!
//! 约定：
//! - 调用处一律传入**英文原文**作为 key（例如 `tr("Actions")`）。
//! - 游戏固定系统/货币专有名词按产品决策保留英文（HEXA / Sol Erda / Erda /
//!   Familiar / Buff / Minimap / Booster 等），其映射 value 与 key 相同。
//!
//! 扩展方式：新增译文只需在下方 `match` 中追加 `"English" => "中文"` 分支。
//! 将来若要支持多语言切换，可把映射抽成 `.toml`/`.ron` 数据文件并用 `include_str!` 内嵌，
//! 此处接口（`tr`）保持不变。

/// 翻译用户可见文本。传入英文原文作为 key，返回对应译文（默认简体中文）。
pub fn tr<'a>(key: &'a str) -> &'a str {
    match key {
        "(empty)" => "(empty)",
        "(no priority actions)" => "(无优先动作)",
        "⏱︎  - Wait" => "⏱︎  - 等待",
        "⁺ - Buffered wait after" => "⁺ - 缓冲后等待",
        "⟳ - Repeat" => "⟳ - 重复",
        "▶" => "▶",
        "⇈ - Queue to front" => "⇈ - 排到最前",
        "→ - Right direction" => "→ - 右方向",
        "← - Left direction" => "← - 左方向",
        "⇆ - Any direction" => "⇆ - 任意方向",
        "① Queues" => "① 队列",
        "② Conditions & Cooldown" => "② 条件与冷却",
        "2x EXP Coupon" => "2x EXP Coupon",
        "③ Timeline" => "③ 时间线",
        "3x EXP Coupon" => "3x EXP Coupon",
        "④ Player State" => "④ 玩家状态",
        "4x EXP Coupon" => "4x EXP Coupon",
        "50% Bonus EXP Coupon" => "50% Bonus EXP Coupon",
        "A ~ B - Random range between A and B" => "A ~ B - A 与 B 之间的随机范围",
        "A ⤓ - Key A is held down" => "A ⤓ - 按键 A 被按住",
        "A ↝ B - Use A key then B key" => "A ↝ B - 先使用 A 键再使用 B 键",
        "A ↜ B - Use B key then A key" => "A ↜ B - 先使用 B 键再使用 A 键",
        "A ↭ B - Use A and B keys at the same time" => "A ↭ B - 同时使用 A 和 B 键",
        "A ↷ B - Use A key then B key while A is held down" => {
            "A ↷ B - 在按住 A 键的同时先使用 A 键再使用 B 键"
        }
        "Action legends" => "动作图例",
        "Actions" => "动作",
        "Add" => "添加",
        "Add a new {name} action" => "添加新 {name} 动作",
        "Add a new fixed action" => "添加新固定动作",
        "Add action" => "添加动作",
        "Add move action" => "添加移动动作",
        "Add platform" => "添加平台",
        "Adjust" => "调整",
        "After the last key use, instead of waiting inplace, the bot is allowed to execute the next action partially. This can be useful for movable skill with casting animation." => {
            "在最后一次按键使用后，bot 不再原地等待，而是允许部分执行下一个动作。这对带有施法动画的可移动技能有用。"
        }
        "All popups." => "所有弹窗。",
        "Amount" => "数量",
        "Any" => "任意",
        "Applicable only for mage class when teleport range increase buff is turned on." => {
            "仅当法师职业且开启传送距离增加 buff 时适用。"
        }
        "Applicable only for non-mage class and when non-up-arrow up jump key is set." => {
            "仅当非法师职业且设置了非上箭头的二段跳按键时适用。"
        }
        "Applicable only if grapple key is set." => "仅当设置了抓钩按键时适用。",
        "Applicable only to mage class or when non-up-arrow up jump key is set." => {
            "仅当法师职业或设置了非上箭头的二段跳按键时适用。"
        }
        "Applicable only to mage class." => "仅适用于法师职业。",
        "Aurelia's Elixir" => "Aurelia's Elixir",
        "Auto mobbing uses key when pathing" => "自动刷怪在路径移动时使用按键",
        "Auto-mobbing pathing" => "自动刷怪路径",
        "Buffs" => "Buffs",
        "Can swap epic familiars" => "Can swap epic familiars",
        "Can swap rare familiars" => "Can swap rare familiars",
        "Cancel" => "取消",
        "Cancel (new)" => "取消 (新)",
        "Cancel (old)" => "取消 (旧)",
        "Capture" => "捕获",
        "Capture color" => "捕获颜色",
        "Capture grayscale" => "捕获灰度",
        "Cash shop" => "现金商店",
        "Cash shop is used to reset spin rune to a normal rune. This only happens if solving rune fails 8 times consecutively." => {
            "现金商店用于将旋转符文重置为普通符文。仅当解符文连续失败 8 次时发生。"
        }
        "Cash shop text." => "现金商店文本。",
        "Change channel" => "切换频道",
        "Change channel text." => "切换频道文本。",
        "Click to set" => "点击设置",
        "Confirm" => "确认",
        "Confirm popup." => "确认弹窗。",
        "Convert button" => "转换按钮",
        "Convert button." => "转换按钮。",
        "Convert Sol Erda to HEXA Booster." => "将 Sol Erda 转换为 HEXA Booster。",
        "Copy" => "复制",
        "Count" => "次数",
        "Create" => "创建",
        "Create a character..." => "创建角色...",
        "Create a map..." => "创建地图...",
        "Create an actions preset for the selected map..." => "为所选地图创建动作预设...",
        "Debug" => "调试",
        "Default" => "默认",
        "Delete" => "删除",
        "Detect lie detector event." => "检测测谎仪事件。",
        "Detect mobs when pathing every" => "路径移动时每间隔检测怪物",
        "Detect whether change channel menu is opened." => "检测是否打开了切换频道菜单。",
        "Detect whether Generic/HEXA booster is in use." => {
            "检测 Generic/HEXA booster 是否在使用中。"
        }
        "Detect whether player entered cash shop." => "检测玩家是否进入现金商店。",
        "Detected size" => "检测到的尺寸",
        "Detection fails or map changes" => "检测失败或地图变化",
        "Disable" => "禁用",
        "Disable double jumping" => "禁用二段跳",
        "Disable grapple on double jumping" => "二段跳时禁用抓钩",
        "Disable teleport on fall" => "下落时禁用传送",
        "Disable walking" => "禁用行走",
        "Discord ping user ID" => "Discord 提醒用户 ID",
        "Double jump" => "二段跳",
        "Duration (hh:mm:ss)" => "时长 (hh:mm:ss)",
        "Elite boss spawns" => "精英怪刷新",
        "Elite boss spawns behavior" => "精英怪刷新行为",
        "Enable" => "启用",
        "Enable panic mode" => "启用紧急模式",
        "Enable rune solving" => "启用的符文解算",
        "Enable transparent shape solving" => "启用透明形态解算",
        "Enable Violetta solving" => "启用 Violetta 解算",
        "Enabled" => "已启用",
        "End chat" => "结束对话",
        "Enter a name..." => "输入名称...",
        "Erda conversion button" => "Erda 转换按钮",
        "Erda conversion button." => "Erda 转换按钮。",
        "Erda Shower off cooldown" => "Erda Shower 冷却结束",
        "Erda Shower off cooldown priority actions" => "Erda Shower 冷却结束优先动作",
        "Every (mm:ss)" => "每 (mm:ss)",
        "every milliseconds" => "每毫秒",
        "Every milliseconds priority actions" => "每毫秒优先动作",
        "Exchange all" => "全部兑换",
        "Exchange when Sol Erda" => "当 Sol Erda 时兑换",
        "EXP Accumulation Potion" => "EXP Accumulation Potion",
        "Export" => "导出",
        "Extreme Blue Potion" => "Extreme Blue Potion",
        "Extreme Gold Potion" => "Extreme Gold Potion",
        "Extreme Green Potion" => "Extreme Green Potion",
        "Extreme Red Potion" => "Extreme Red Potion",
        "Familiar essence" => "Familiar essence",
        "Familiar menu" => "Familiar menu",
        "Familiar menu setup tab's save button." => "Familiar 菜单设置标签页的保存按钮。",
        "Familiar menu setup tab's setup level sort button." => {
            "Familiar 菜单设置标签页的等级排序按钮。"
        }
        "Familiar skill" => "Familiar skill",
        "Familiars" => "Familiars",
        "Feed key" => "喂食按键",
        "Fixed actions" => "固定动作",
        "For The Guild" => "为了公会",
        "Friend appears" => "好友出现",
        "Function" => "功能",
        "Generic Booster key" => "Generic Booster 按键",
        "Go to town confirmation and save familiars setup." => {
            "前往城镇确认并保存 familiars 设置。"
        }
        "Guildie appears" => "公会成员出现",
        "Handle" => "句柄",
        "Hard Hitter" => "Hard Hitter",
        "Has extended teleport range" => "拥有扩展传送距离",
        "Height" => "高度",
        "HEXA Booster button" => "HEXA Booster 按钮",
        "HEXA Booster button." => "HEXA Booster 按钮。",
        "HEXA Booster key" => "HEXA Booster 按键",
        "hh:mm:ss" => "hh:mm:ss",
        "Hold for" => "按住时长",
        "Holding buffered" => "保持缓冲",
        "Hotkeys" => "快捷键",
        "HP" => "HP",
        "HP below" => "HP 低于",
        "HP update every" => "HP 更新间隔",
        "ignoring:✓" => "忽略:✓",
        "ignoring:✗" => "忽略:✗",
        "Import" => "导入",
        "Import/export actions" => "导入/导出动作",
        "Info" => "信息",
        "Input" => "输入",
        "Input method" => "输入方式",
        "Interact" => "交互",
        "Jump" => "跳跃",
        "Jump then up jump if possible" => "若可能则跳跃后接二段跳",
        "Key" => "按键",
        "Key bindings" => "按键绑定",
        "Key to use" => "使用按键",
        "Legion's Luck" => "Legion's Luck",
        "Legion's Wealth" => "Legion's Wealth",
        "Level sort button" => "等级排序按钮",
        "Lie detector (new)" => "测谎仪 (新)",
        "Lie detector (old)" => "测谎仪 (旧)",
        "Lie detector appears" => "测谎仪出现",
        "Lie detector title." => "测谎仪标题。",
        "Link key" => "连接按键",
        "Link key timing" => "连接按键时机",
        "Link key type" => "连接按键类型",
        "linked" => "已连接",
        "Linked action" => "连接动作",
        "Mark platform end" => "标记平台结束",
        "Mark platform start" => "标记平台开始",
        "Max button" => "最大按钮",
        "Max button." => "最大按钮。",
        "Method" => "方式",
        "mm:ss" => "mm:ss",
        "Mode" => "模式",
        "Modify a {name} action" => "修改 {name} 动作",
        "Modify a fixed action" => "修改固定动作",
        "Modify mobbing bound" => "修改刷怪边界",
        "Modify mobbing key" => "修改刷怪按键",
        "Modify platform" => "修改平台",
        "Move tolerance" => "移动容差",
        "Movement" => "移动",
        "Next" => "下一步",
        "No minimap detected" => "未检测到 minimap",
        "None" => "无",
        "normal" => "普通",
        "Normal action" => "普通动作",
        "Normal actions" => "普通动作",
        "Not applicable if an action requires adjusting." => "若动作需要调整则不适用。",
        "Not applicable if an action requires double jumping." => "若动作需要二段跳则不适用。",
        "Notifications" => "通知",
        "Ok (new)" => "确定 (新)",
        "Ok (new) popup." => "确定 (新) 弹窗。",
        "Ok (old)" => "确定 (旧)",
        "Open HEXA Booster exchange menu." => "打开 HEXA Booster 兑换菜单。",
        "Open Sol Erda version menu in HEXA Matrix." => "在 HEXA Matrix 中打开 Sol Erda 版本菜单。",
        "Others" => "其他",
        "Pathing means when the player is moving from one quad to another." => {
            "路径移动指玩家从一个区块移动到另一个区块。"
        }
        "Pause" => "暂停",
        "Pixel radius for considering a move as arrived. Values above 25 for normal classes or 12 for mages may cause movement issues, as the character may double jump within the tolerance instead of moving along the Y axis." => {
            "判定移动已到达的像素半径。普通职业超过 25 或法师超过 12 可能导致移动问题，因为角色可能在容差范围内二段跳而非沿 Y 轴移动。"
        }
        "Platforms" => "平台",
        "Player dies" => "玩家死亡",
        "Popups" => "弹窗",
        "Position" => "位置",
        "Positioned" => "已就位",
        "Potion key" => "药水按键",
        "Press any key..." => "按任意键...",
        "Priority" => "优先级",
        "Priority action" => "优先动作",
        "Queue to front" => "排到最前",
        "Re-detect" => "重新检测",
        "Refresh handles" => "刷新句柄",
        "Replace" => "替换",
        "Require [Wait after buffered] to be enabled and without [Link key]. When enabled, the holding time will be added to [Wait after] during the last key use. Useful for holding down key and moving simultaneously." => {
            "需要启用 [Wait after buffered] 且未使用 [Link key]。启用后，在最后一次按键使用时，按住时长会加到 [Wait after] 上。适用于同时按住按键并移动的场景。"
        }
        "Requires HEXA Booster to be visible in quick slots, Sol Erda tracker menu opened and HEXA Matrix configured in the quick menu. Exchange will only happen if there is no HEXA Booster." => {
            "需要 HEXA Booster 在快捷栏可见、Sol Erda 追踪菜单已打开且 HEXA Matrix 已在快捷菜单中配置。仅当没有 HEXA Booster 时才会兑换。"
        }
        "Reset" => "重置",
        "Reset normal actions on Erda Shower resets" => "Erda Shower 重置时重置普通动作",
        "Respawn on player death." => "玩家死亡时重生。",
        "Resume" => "继续",
        "Rope lift" => "绳索升降",
        "Rotation" => "轮换",
        "Rotator Debug" => "轮换器调试",
        "RPC server URL" => "RPC 服务器 URL",
        "Run timer" => "运行计时器",
        "Run timer ends" => "运行计时器结束",
        "Rune pathing" => "符文路径",
        "Rune spawns" => "符文刷新",
        "Save" => "保存",
        "Save button" => "保存按钮",
        "Save familiars setup after swapping." => "交换后保存 familiars 设置。",
        "Sayram's Elixir" => "Sayram's Elixir",
        "Section" => "区块",
        "Select max HEXA Booster amount to exchange." => "选择要兑换的最大 HEXA Booster 数量。",
        "Selected size" => "选中的尺寸",
        "Side" => "方向",
        "Small EXP Accumulation Potion" => "Small EXP Accumulation Potion",
        "Small Wealth Acquisition Potion" => "Small Wealth Acquisition Potion",
        "Sort familiar cards by level before swapping." => "交换前按等级排序 familiar 卡片。",
        "Start auto record lie detector" => "开始自动记录测谎仪",
        "Start auto saving rune" => "开始自动保存符文",
        "Start recording" => "开始录制",
        "State" => "状态",
        "Stationary" => "静止",
        "Stop actions on fail or map changed" => "失败或地图变化时停止动作",
        "Stop actions on player dies" => "玩家死亡时停止动作",
        "Stop auto record lie detector" => "停止自动记录测谎仪",
        "Stop auto saving rune" => "停止自动保存符文",
        "Stop recording" => "停止录制",
        "Stranger appears" => "陌生人出现",
        "Swap check every (mm:ss)" => "交换检查间隔 (mm:ss)",
        "Swappable slots" => "可交换槽位",
        "Swapping enabled" => "交换已启用",
        "Switch to key" => "切换到按键",
        "Switch to move" => "切换到移动",
        "Teleport" => "传送",
        "Template(s)" => "模板",
        "Test spin rune" => "测试旋转符文",
        "Test transparent shape hard" => "测试透明形态（困难）",
        "Test transparent shape normal" => "测试透明形态（普通）",
        "Test Violetta" => "测试 Violetta",
        "This is meant for classes that have a separate skill to up jump. Classes that use up arrow should set this key to up arrow." => {
            "这适用于拥有独立二段跳技能的职业。使用上箭头的职业应将此按键设为上箭头。"
        }
        "This key must be set to use familiars swapping feature." => {
            "必须使用此按键才能使用 familiars 交换功能。"
        }
        "This key must be set to use navigation or run/stop cycle features." => {
            "必须使用此按键才能使用导航或运行/停止循环功能。"
        }
        "This key must be set to use panic mode or elite boss spawns behavior features." => {
            "必须使用此按键才能使用紧急模式或精英怪刷新行为功能。"
        }
        "This template is in grayscale." => "此模板为灰度图。",
        "Timer" => "计时器",
        "Timer text." => "计时器文本。",
        "To town" => "前往城镇",
        "Toggle start/stop actions" => "切换开始/停止动作",
        "Unknown" => "未知",
        "Unstuck player through closing menu, popup, dialog, etc." => {
            "通过关闭菜单、弹窗、对话框等解除玩家卡死。"
        }
        "Up jump" => "二段跳",
        "Up jump is flight" => "二段跳即飞行",
        "Up jump only" => "仅二段跳",
        "Update" => "更新",
        "Update mobbing bound" => "更新刷怪边界",
        "Update mobbing key" => "更新刷怪按键",
        "Use booster" => "使用 booster",
        "Use count" => "使用次数",
        "Use direction" => "使用方向",
        "Use every" => "每间隔使用",
        "Use potion and feed pet" => "使用药水并喂养宠物",
        "Use with" => "配合使用",
        "Wait after buffered" => "缓冲后等待",
        "Wait after move" => "移动后等待",
        "Wait after use" => "使用后等待",
        "Wait before use" => "使用前置等待",
        "Wait random range" => "随机等待范围",
        "Wealth Acquisition Potion" => "Wealth Acquisition Potion",
        "Webhook provider" => "Webhook 提供方",
        "Webhook URL" => "Webhook URL",
        "Width" => "宽度",
        "X" => "X",
        "X end" => "X 结束",
        "X offset" => "X 偏移",
        "X random range" => "X 随机范围",
        "X range" => "X 范围",
        "X start" => "X 开始",
        "Y" => "Y",
        "Y offset" => "Y 偏移",
        "Yes" => "是",
        "ㄨ - No position" => "ㄨ - 无位置",
        other => other,
    }
}
