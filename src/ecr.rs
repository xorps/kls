const EAST_ENDPONT: &str = ".dkr.ecr.us-east-1.amazonaws.com";
const WEST_ENDPOINT: &str = ".dkr.ecr.us-west-2.amazonaws.com";

pub fn is_ecr(image: &str) -> bool {
    if let Some(pos) = image.find("/") {
        let uri = &image[..pos];
        uri == "public.ecr.aws" || uri.ends_with(EAST_ENDPONT) || uri.ends_with(WEST_ENDPOINT)
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::is_ecr;

    #[test]
    fn test_is_docker() {
        assert!(is_ecr("public.ecr.aws/docker/library/alpine:latest",));
        assert!(is_ecr(
            "112321.dkr.ecr.us-east-1.amazonaws.com/my-app:latest",
        ));
        assert!(!is_ecr("nginx:latest"));
        assert!(!is_ecr("registry.k8s.io/metrics-server:latest",));
    }
}
