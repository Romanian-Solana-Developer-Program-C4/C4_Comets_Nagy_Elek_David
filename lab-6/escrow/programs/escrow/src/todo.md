Alice -> Escrow -> Bob
Bob -> Escrow -> Alice

Alice: 1 SOL -> 100 USDC
Bob: 100 USDC -> 1 SOL

Alice: -> make offer:

Offer {
    token_a_mint: address
    token_b_mint: address
    token_a_amount: u64
    token_b_amount: u64
}

Bob: -> take_offer
        -> 100 USDC Alice
        <- 1 SOL Bob
