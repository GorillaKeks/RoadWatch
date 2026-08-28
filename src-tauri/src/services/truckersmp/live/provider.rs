use std::future::Future;
use std::pin::Pin;

use crate::models::LivePlayer;

/// A source that can provide live TruckersMP player data.
///
/// Implementations may support ETS2, ATS or both games.
pub trait LivePlayerProvider: Send + Sync {
    fn name(&self) -> &'static str;

    fn get_live_players(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<Vec<LivePlayer>, String>,
                > + Send
                + '_,
        >,
    >;
}