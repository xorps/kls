use std::convert::identity;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    target: Kind,
}

#[derive(Clone, Debug, ValueEnum)]
enum Kind {
    Pod,
    Workload,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run(args))
}

async fn run(arg: Args) -> anyhow::Result<()> {
    let client = kube::Client::try_default().await?;

    match arg.target {
        Kind::Pod => {
            let _pods = list_pods(client).await?;
            Ok(())
        }
        _ => Ok(())
    }
}

struct Pod {
    namespace: String,
    name: String,
    image: String,
}

async fn list_pods(client: kube::Client) -> anyhow::Result<Vec<Pod>> {
    let api: kube::Api<k8s_openapi::api::core::v1::Pod> = kube::Api::all(client);

    let pods = api.list(&Default::default()).await?;

    let pods: Vec<Pod> = pods
        .items
        .into_iter()
        .flat_map(|item| {
            let name = item.metadata.name.unwrap_or_default();
            let namespace = item.metadata.namespace.unwrap_or_default();
            item.spec
                .map(|spec| {
                    let init_containers = spec
                        .init_containers
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|i| is_docker_hub(i));
                    let containers = spec
                        .containers
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|i| is_docker_hub(i));
                    let c = init_containers.chain(containers);
                    let c: Vec<_> = c
                        .map(|image| Pod {
                            namespace: namespace.clone(),
                            name: name.clone(),
                            image,
                        })
                        .collect();
                    c
                })
                .unwrap_or_default()
        })
        .collect();

    Ok(pods)
}

fn is_docker_hub(image: &str) -> bool {
    true
}
