# FeldmanVSS note

## 历史

1. SSS 由Shamir提出密钥共享及一个基于多项式插值的方案
2. VSS 由Chor, Goldwasser, Micali, Awerbuch提出可验证密钥共享，并提出一个基于大数分解难题的常数轮交互方案
3. 基于Goldreich, Micali和Wigderson的零知识证明系统，可以构造常数轮交互方案
4. Benaloh基于可靠公共信标，构造出常数轮交互方案

5.feldman在本文提出了第一个非交互的方案，仅需要两轮通信。

## 实用性指标

1. 通信的轮数
2. 通信的数据大小

3.dealer需要执行的计算量

## FeldmanVSS 协议

定义：
(Share, Recover, Check, Encrypt)

### 工具

1. 概率加密

确定性加密，存在选择明文攻击(CPA)。

2. 同态加密函数

### 步骤  

1. 初始化

2. 密钥分享

3. 密钥恢复

## 应用

### 模拟一个同步广播网络

### 快速拜占庭一致性

### 密码学协议的组件

