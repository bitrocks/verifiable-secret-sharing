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
3. dealer需要执行的计算量

## FeldmanVSS 协议

<!-- 定义：
(Share, Recover, Check, Encrypt)

### 工具

1. 概率加密

确定性加密，存在选择明文攻击(CPA)。

2. 同态加密函数 -->

### 步骤  

1. 初始化
    1. 椭圆曲线(如Secp256k1)；
    2. 随机数生成器；

2. 密钥分享(dealer)
    1. 输入{secret, k, n}，secret在内部转换为`Secp256k1Scalar`；
    2. 生成多项式函数的系数coefs = {a0,a1,a2,...,ak-1}，表示一个k-1次多项式，
    <img src="https://render.githubusercontent.com/render/math?math=a(x) = a_0  %2B a_1x %2B a_2x^2 %2B \cdots %2B a_{k-1}x^{k-1}">
    ，a0 = secret，需要k个点的数据才能恢复；
    3. 在多项式上取n个点，生成n个子密钥，{<1, a(1)>, <2,a(2)>, ..., <n, a(n)>}，计算过程x值全部转化成`Secp256k1Scalar`；
    4. 生成k个系数的commitments，
    <img src="https://render.githubusercontent.com/render/math?math=c(i) = g^{a_i}">，其中g是椭圆曲线上的的generator

3. 密钥恢复(dealer)
    1. 任意k个点，使用Lagrange插值恢复出secret；
    2. 验证 c0 == g^secret

4. 子密钥验证(players)
    1. 任何子密钥持有者执行以下校验，{<i, a(i)>, {c0,c1,c2,..ck-1}}
    <img src="https://render.githubusercontent.com/render/math?math=g^{a_i} = g^{a_0  %2B a_1i %2B a_2i^2 %2B \cdots %2B a_{k-1}i^{k-1}} = c_0 c_1^ic_2i^2...c_{k-1}i^{k-1}">
    通过验证表明dealer分配的子密钥是正确的。

## 应用

### 模拟一个同步广播网络

### 快速拜占庭一致性

### 密码学协议的组件



































































