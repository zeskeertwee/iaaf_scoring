use iaaf_scoring;
use iaaf_scoring::methods::ScoringMethod;
use iaaf_scoring::preformance::Preformance;

fn main() {
    pretty_env_logger::init();
    let mut table = iaaf_scoring::methods::iaaf_2017_tables_outdoor::Iaaf2017Tables::new();
    table.load_all_tables();

    let points = table
        .score(
            iaaf_scoring::event::OutdoorEvent::HighJump,
            iaaf_scoring::gender::Gender::Female,
            Preformance::Distance(1.64),
        )
        .unwrap();

    log::info!(
        "Points for {} HJ FEMALE: {}",
        Preformance::Distance(1.64),
        points
    );
}
