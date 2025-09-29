use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "src/abi/ERC20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Cex,
    "src/abi/Cex.json"
);
