#[derive(Debug, Clone, Copy)]
pub struct LiveArea {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

/// Confirmed working TruckersMP/ETS2Map global ETS2 area.
pub const GLOBAL_ETS2_AREA: LiveArea = LiveArea {
    x1: -88026,
    y1: 140789,
    x2: 72774,
    y2: -106596,
};

/// Global ATS area used for TruckersMP live player queries.
///
/// This range covers the currently known ATS coordinate range
/// and includes southern map regions that were outside the
/// previous RoadWatch query area.
pub const GLOBAL_ATS_AREA: LiveArea = LiveArea {
    x1: -250000,
    y1: 150000,
    x2: 100000,
    y2: -150000,
};