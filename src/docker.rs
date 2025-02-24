/*
pub fn is_docker_hub(image: &str) -> bool {
    if image.starts_with("docker.io") {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use crate::docker::is_docker_hub;

    #[test]
    fn test_is_docker() {
        assert!(is_docker_hub("nginx:latest"));
        assert!(is_docker_hub("python:3.9.21-alpine3.2@sha256:fccb70daee4ed25a6adf99d37562a42e4c6e1ef97f5dbfcc3b70e1fd184ce714"));
        assert!(is_docker_hub("docker.io/library/python:3.9.21-alpine3.2@sha256:fccb70daee4ed25a6adf99d37562a42e4c6e1ef97f5dbfcc3b70e1fd184ce714"));
        assert!(!is_docker_hub(
            "public.ecr.aws/docker/library/python:3.9.21-alpine3.21"
        ));
        assert!(!is_docker_hub("public.ecr.aws/docker/library/python:3.9.21-alpine3.21@@sha256:fccb70daee4ed25a6adf99d37562a42e4c6e1ef97f5dbfcc3b70e1fd184ce714"));
        assert!(!is_docker_hub("foo.bar"));
    }
}
*/
