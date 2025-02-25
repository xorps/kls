mod ecr;
mod k8s;

use clap::{Parser, ValueEnum};
use ecr::is_ecr;
use k8s::{list_daemonsets, list_deployments, list_pods, list_statefulsets};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    target: Kind,

    #[arg(short, long, value_enum, default_value_t = Filter::All)]
    filter: Filter,
}

#[derive(Clone, Debug, ValueEnum)]
enum Kind {
    Pod,
    Workload,
}

#[derive(Clone, Debug, ValueEnum)]
enum Filter {
    All,
    NotEcr,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(run(args))
}

async fn run(arg: Args) -> anyhow::Result<()> {
    let client = kube::Client::try_default().await?;

    let filter = match arg.filter {
        Filter::All => |_: &str| true,
        Filter::NotEcr => |image: &str| !is_ecr(image, ecr::Region::UsEast1),
    };

    match arg.target {
        Kind::Pod => {
            let pods = list_pods(client, filter).await?;
            let pods = pods
                .into_iter()
                .map(|p| p.show())
                .collect::<Vec<_>>()
                .join("\n");
            println!("Pods:\n{}", pods);
            Ok(())
        }
        Kind::Workload => {
            let (deploy, ds, sts) = tokio::try_join!(
                list_deployments(client.clone(), filter),
                list_daemonsets(client.clone(), filter),
                list_statefulsets(client, filter),
            )?;
            let deploy = deploy
                .into_iter()
                .map(|d| d.show())
                .collect::<Vec<_>>()
                .join("\n");
            let ds = ds
                .into_iter()
                .map(|d| d.show())
                .collect::<Vec<_>>()
                .join("\n");
            let sts = sts
                .into_iter()
                .map(|d| d.show())
                .collect::<Vec<_>>()
                .join("\n");
            println!("Deployments:\n{}\n", deploy);
            println!("DaemonSets:\n{}\n", ds);
            println!("StateFulSets:\n{}\n", sts);
            Ok(())
        }
    }
}
