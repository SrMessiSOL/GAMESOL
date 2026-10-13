use anchor_lang::prelude::*;

use crate::types::ResourceType;

#[account]
#[derive(InitSpace)]
pub struct MarketConfig {
    pub admin: Pubkey,
    pub antimatter_mint: Pubkey,
    pub total_volume: u128,
    pub total_offers: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct MarketOffer {
    pub seller: Pubkey,
    pub seller_planet: Pubkey,
    pub resource_type: ResourceType,
    pub resource_amount: u64,
    pub price_antimatter: u64,
    pub created_at: i64,
    pub offer_id: u32,
    pub filled: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PlanetListing {
    pub seller: Pubkey,
    pub planet: Pubkey,
    pub planet_coords: Pubkey,
    pub price_antimatter: u64,
    pub created_at: i64,
    pub listing_id: u32,
    pub filled: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PlanetListingIndex {
    pub planet: Pubkey,
    pub listing: Pubkey,
    pub seller: Pubkey,
    pub active: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PlanetMarketObligation {
    pub planet: Pubkey,
    pub active_resource_offers: u32,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct SellerCounter {
    pub seller: Pubkey,
    pub next_offer_id: u32,
    pub active_offers: u32,
    pub bump: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sponsored_market_rent_spaces_match_client_contract() {
        assert_eq!(8 + SellerCounter::INIT_SPACE, 49);
        assert_eq!(8 + MarketOffer::INIT_SPACE, 103);
        assert_eq!(8 + PlanetMarketObligation::INIT_SPACE, 45);
        assert_eq!(8 + PlanetListing::INIT_SPACE, 126);
        assert_eq!(8 + PlanetListingIndex::INIT_SPACE, 106);
    }
}
