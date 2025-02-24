pub struct Pod {
    namespace: String,
    name: String,
    image: String,
}

impl Pod {
    pub fn show(&self) -> String {
        format!("{}/{} [{}]", self.namespace, self.name, self.image)
    }
}

pub struct Deployment {
    namespace: String,
    name: String,
    image: String,
}

impl Deployment {
    pub fn show(&self) -> String {
        format!("{}/{} [{}]", self.namespace, self.name, self.image)
    }
}

pub struct StatefulSet {
    namespace: String,
    name: String,
    image: String,
}

impl StatefulSet {
    pub fn show(&self) -> String {
        format!("{}/{} [{}]", self.namespace, self.name, self.image)
    }
}

pub struct DaemonSet {
    namespace: String,
    name: String,
    image: String,
}

impl DaemonSet {
    pub fn show(&self) -> String {
        format!("{}/{} [{}]", self.namespace, self.name, self.image)
    }
}

pub async fn list_pods<Filter>(client: kube::Client, filter: Filter) -> anyhow::Result<Vec<Pod>>
where
    Filter: Fn(&str) -> bool,
{
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
                        .filter(|img| filter(img));
                    let containers = spec
                        .containers
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
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

pub async fn list_deployments<Filter>(
    client: kube::Client,
    filter: Filter,
) -> anyhow::Result<Vec<Deployment>>
where
    Filter: Fn(&str) -> bool,
{
    let api: kube::Api<k8s_openapi::api::apps::v1::Deployment> = kube::Api::all(client);

    let deployments = api.list(&Default::default()).await?;

    let deploys: Vec<Deployment> = deployments
        .items
        .into_iter()
        .flat_map(|item| {
            let name = item.metadata.name.unwrap_or_default();
            let namespace = item.metadata.namespace.unwrap_or_default();
            item.spec
                .and_then(|spec| spec.template.spec)
                .map(|spec| {
                    let init_containers = spec
                        .init_containers
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
                    let containers = spec
                        .containers
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
                    let c = init_containers.chain(containers);
                    let c: Vec<_> = c
                        .map(|image| Deployment {
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

    Ok(deploys)
}

pub async fn list_statefulsets<Filter>(
    client: kube::Client,
    filter: Filter,
) -> anyhow::Result<Vec<StatefulSet>>
where
    Filter: Fn(&str) -> bool,
{
    let api: kube::Api<k8s_openapi::api::apps::v1::StatefulSet> = kube::Api::all(client);

    let sts = api.list(&Default::default()).await?;

    let sts: Vec<StatefulSet> = sts
        .items
        .into_iter()
        .flat_map(|item| {
            let name = item.metadata.name.unwrap_or_default();
            let namespace = item.metadata.namespace.unwrap_or_default();
            item.spec
                .and_then(|spec| spec.template.spec)
                .map(|spec| {
                    let init_containers = spec
                        .init_containers
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
                    let containers = spec
                        .containers
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
                    let c = init_containers.chain(containers);
                    let c: Vec<_> = c
                        .map(|image| StatefulSet {
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

    Ok(sts)
}

pub async fn list_daemonsets<Filter>(
    client: kube::Client,
    filter: Filter,
) -> anyhow::Result<Vec<DaemonSet>>
where
    Filter: Fn(&str) -> bool,
{
    let api: kube::Api<k8s_openapi::api::apps::v1::DaemonSet> = kube::Api::all(client);

    let daemonsets = api.list(&Default::default()).await?;

    let ds: Vec<DaemonSet> = daemonsets
        .items
        .into_iter()
        .flat_map(|item| {
            let name = item.metadata.name.unwrap_or_default();
            let namespace = item.metadata.namespace.unwrap_or_default();
            item.spec
                .and_then(|spec| spec.template.spec)
                .map(|spec| {
                    let init_containers = spec
                        .init_containers
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
                    let containers = spec
                        .containers
                        .into_iter()
                        .filter_map(|c| c.image)
                        .filter(|img| filter(img));
                    let c = init_containers.chain(containers);
                    let c: Vec<_> = c
                        .map(|image| DaemonSet {
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

    Ok(ds)
}
