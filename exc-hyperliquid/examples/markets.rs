use hypersdk::hypercore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a mainnet client
    let client = hypercore::mainnet();

    // Get spot markets
    let spots = client.spot().await?;
    for market in spots {
        println!("{}-{}({})", market.symbol(), market.name, market.index);
    }

    // Get perpetual markets
    let perps = client.perps().await?;
    for market in perps {
        if market.isolated_margin {
            continue;
        }
        println!(
            "{}-{}({}): {}x leverage",
            market.name, market.collateral.name, market.index, market.max_leverage
        );
    }

    // Get markets from a specific DEX
    let dexes = client.perp_dexs().await?;
    for dex in &dexes {
        println!("DEX: {}", dex.name());
        let markets = client.perps_from(dex.clone()).await?;
        for market in markets {
            if market.isolated_margin {
                continue;
            }
            println!(
                "{}-{}({}): {}x leverage",
                market.name, market.collateral.name, market.index, market.max_leverage
            );
        }
    }

    Ok(())
}
