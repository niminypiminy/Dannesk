#[derive(PartialEq, Clone)]  // ← ADD THIS LINE (Clone is nice to have too)
pub struct XrpBalanceInfo {
    pub is_active: bool,
    pub total_reserve: f64,
    pub available: f64,
    pub base_reserve: f64,
    pub trustline_reserve: f64,
}

pub fn get_xrp_balance_info(total_xrp: f64, active_trustline_count: usize) -> XrpBalanceInfo {
    if total_xrp < 1.0 {
        return XrpBalanceInfo {
            is_active: false,
            total_reserve: 0.0,
            available: total_xrp,
            base_reserve: 0.0,
            trustline_reserve: 0.0,
        };
    }

    let base_reserve = 1.0;
    let trustline_reserve = (active_trustline_count as f64) * 0.20;
    let total_reserve = base_reserve + trustline_reserve;
    let available = (total_xrp - total_reserve).max(0.0);

    XrpBalanceInfo {
        is_active: true,
        total_reserve,
        available,
        base_reserve,
        trustline_reserve,
    }
}