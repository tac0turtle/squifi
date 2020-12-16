# Paybacks

## Context

When investing into a raise there is the potential for the investment to provide a return on the investment. This return is distributed to holders of the funds token. 

Fund tokens represents a partial ownership of the fund. 

### Questions?

- When should an investor be allowed to redeem from payback pool?

- How should redemption be tracked?
  - If redemption is not tracked then the same token holder could redeem the entire pool.

  - 1. Track account holders of the token within the contract. To enable transfers of the fund token it would need to happen through the contract.

  - 2. generate nfts for a certain amount of deposit and allow withdrawls that way. 

  - 3. We have two nfts, on initial deposit nft_a is given to the investor on payback nft_a is burned and nft_b is minted to the investor. nft_b can not be used to redeem the current rewards. Once all nft_a tokens are burned nft_b tokens can be used to redeem more of the pool. switching back and forth constantly. As nft tokens are burned then nft_a token is reduced but the shares count stays the same. To calculate the outstanding rewards for a nft_a token holder one can use (my_nft_a)

  - 4. Take a snapshot of all current token holders on the client side and issue token transfers to all of them. This requires users to claim an address that is snapshotted. 
    - rpc request to the token owners for the token mint: [token owner rpc call](https://spl.solana.com/token#finding-all-token-accounts-for-a-specific-mint)
    - check the nft token account owner has an account for the reward token, if not spl.
    - if not use [associated-token-account](https://spl.solana.com/associated-token-account) to generate them an address and send it there.
    - if they do then send it to that wallet and record it. 
      - How should it be recorded?
