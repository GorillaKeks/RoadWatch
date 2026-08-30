use roadwatch_lib::services::truckersmp::server::TruckersMpServerService;

#[tokio::main]
async fn main() {
    println!("RoadWatch TruckersMP Server Test");
    println!("================================");

    let service = TruckersMpServerService::new();

    println!();
    println!("ETS2 servers:");
    println!("-------------");

    match service.get_ets2_servers().await {
        Ok(servers) => {
            println!("{} ETS2 servers found.", servers.len());

            for server in servers {
                println!(
                    "{} | ID: {} | Players: {}/{}",
                    server.name, server.id, server.players, server.max_players
                );
            }
        }

        Err(error) => {
            eprintln!("Failed to load ETS2 servers: {}", error);
        }
    }

    println!();
    println!("ATS servers:");
    println!("------------");

    match service.get_ats_servers().await {
        Ok(servers) => {
            println!("{} ATS servers found.", servers.len());

            for server in servers {
                println!(
                    "{} | ID: {} | Players: {}/{}",
                    server.name, server.id, server.players, server.max_players
                );
            }
        }

        Err(error) => {
            eprintln!("Failed to load ATS servers: {}", error);
        }
    }
}
