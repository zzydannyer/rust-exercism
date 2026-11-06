# Instructions

# 指示

Diffie-Hellman key exchange.

Diffie-Hellman 密钥交换。

Alice and Bob use Diffie-Hellman key exchange to share secrets.
They start with prime numbers, pick private keys, generate and share public keys, and then generate a shared secret key.

Alice 和 Bob 使用 Diffie-Hellman 密钥交换来共享秘密。
他们从素数开始，选择私钥，生成并共享公钥，然后生成共享密钥。

## Step 0

## 步骤 0

The test program supplies prime numbers p and g.

测试程序提供素数 p 和 g。

## Step 1

## 步骤 1

Alice picks a private key, a, greater than 1 and less than p.
Bob does the same to pick a private key b.

Alice 选择一个私钥 a，大于 1 且小于 p。
Bob 执行同样的操作来选择私钥 b。

## Step 2

## 步骤 2

Alice calculates a public key A.

Alice 计算出公钥 A。

    A = gᵃ mod p

Using the same p and g, Bob similarly calculates a public key B from his private key b.

使用相同的 p 和 g，Bob 类似地从他的私钥 b 计算出公钥 B。

## Step 3

## 步骤 3

Alice and Bob exchange public keys.
Alice calculates secret key s.

爱丽丝和鲍勃交换公钥。
Alice 计算秘密密钥 s。

    s = Bᵃ mod p

Bob calculates

鲍勃计算

    s = Aᵇ mod p

The calculations produce the same result!
Alice and Bob now share secret s.


计算得出相同的结果！
Alice 和 Bob 现在共享秘密 s。