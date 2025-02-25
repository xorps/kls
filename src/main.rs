mod ecr;
mod k8s;
mod result;

use clap::{Parser, ValueEnum};
use ecr::is_ecr;
use futures::{FutureExt, TryFutureExt};
use k8s::{list_daemonsets, list_deployments, list_pods, list_statefulsets};
use result::flatten;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target
    #[arg(short, long, value_enum)]
    target: Kind,

    /// Filter
    #[arg(short, long, value_enum, default_value_t = Filter::All)]
    filter: Filter,

    /// Runtime
    #[arg(short, long, value_enum, default_value_t = Runtime::Ct)]
    runtime: Runtime,
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

#[derive(Clone, Debug, ValueEnum)]
enum Runtime {
    /// Current Thread
    Ct,
    /// Multi Thread
    Mt,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.runtime {
        Runtime::Ct => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(run(args)),
        Runtime::Mt => tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(run(args)),
    }
}

async fn run(arg: Args) -> anyhow::Result<()> {
    let client = kube::Client::try_default().await?;

    let filter = match arg.filter {
        Filter::All => |_: &str| true,
        Filter::NotEcr => |image: &str| !is_ecr(image),
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
            let (deploy, ds, sts) = match arg.runtime {
                Runtime::Ct => tokio::try_join!(
                    list_deployments(client.clone(), filter),
                    list_daemonsets(client.clone(), filter),
                    list_statefulsets(client, filter),
                )?,
                Runtime::Mt => tokio::try_join!(
                    tokio::spawn(list_deployments(client.clone(), filter))
                        .err_into::<anyhow::Error>()
                        .map(flatten),
                    tokio::spawn(list_daemonsets(client.clone(), filter))
                        .err_into::<anyhow::Error>()
                        .map(flatten),
                    tokio::spawn(list_statefulsets(client, filter))
                        .err_into::<anyhow::Error>()
                        .map(flatten),
                )?,
            };
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
