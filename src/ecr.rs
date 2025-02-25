use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum Region {
    UsEast1,
}

pub fn is_ecr(image: &str, region: Region) -> bool {
    let endpoint = match region {
        Region::UsEast1 => ".dkr.ecr.us-east-1.amazonaws.com",
    };
    if let Some(pos) = image.find("/") {
        let uri = &image[..pos];
        uri == "public.ecr.aws" || uri.ends_with(endpoint)
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::is_ecr;
    use super::Region;

    #[test]
    fn test_is_docker() {
        assert!(is_ecr(
            "public.ecr.aws/docker/library/alpine:latest",
            Region::UsEast1
        ));
        assert!(is_ecr(
            "112321.dkr.ecr.us-east-1.amazonaws.com/my-app:latest",
            Region::UsEast1
        ));
        assert!(!is_ecr("nginx:latest", Region::UsEast1));
        assert!(!is_ecr(
            "registry.k8s.io/metrics-server:latest",
            Region::UsEast1
        ));
    }
}
