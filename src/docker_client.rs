use async_trait::async_trait;
use docker_api::{
    opts::{ContainerListOpts, ContainerStopOpts, ContainerRestartOpts},
    Docker,
};

const DEFAULT_DOCKER_SOCKET: &str = "unix:///var/run/docker.sock";

pub struct DockerClient {
    docker: Docker,
    container: String,
}

pub trait DockerClientTrait {
    fn new(container: &String) -> DockerClient;
}

#[async_trait]
pub trait AsyncDockerClientTrait {
    async fn healthcheck(&self) -> Result<String, String>;
    async fn start(&self) -> Result<(), String>;
    async fn stop(&self) -> Result<(), String>;
    async fn restart(&self) -> Result<(), String>;
}

impl DockerClientTrait for DockerClient {
    fn new(container: &String) -> DockerClient {
        let docker = docker_api::Docker::new(DEFAULT_DOCKER_SOCKET).unwrap();

        DockerClient {
            docker,
            container: container.into(),
        }
    }
}

#[async_trait]
impl AsyncDockerClientTrait for DockerClient {
    async fn healthcheck(&self) -> Result<String, String> {
        let container_opts = ContainerListOpts::builder().all(true).build();

        // TODO: Change logic to use Container.get and Container.inspect
        let containers = self
            .docker
            .containers()
            .list(&container_opts)
            .await
            .unwrap();

        let container_summary = containers.iter().find(|container| {
            let container_names = container.names.as_ref();

            let res = container_names.into_iter().find(|name| {
                let mut formatted_name: String = name.iter().cloned().collect();

                formatted_name.remove(0);

                return formatted_name.contains(&self.container);
            });

            res.is_some()
        });

        match container_summary {
            Some(container) => {
                let container_status = container.status.as_ref();

                match container_status {
                    Some(status) => Ok(status.into()),
                    None => Err(format!("Container {} has no status", self.container)),
                }
            }
            None => Err(format!("Container {} not found", self.container)),
        }
    }

    async fn start(&self) -> Result<(), String> {
        let container = self.docker.containers().get(&self.container);

        match container.start().await {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Container {} failed to start", self.container)),
        }
    }

    async fn restart(&self) -> Result<(), String> {
        let container = self.docker.containers().get(&self.container);

        let restart_opts = ContainerRestartOpts::builder().build();

        match container.restart(&restart_opts).await {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Container {} failed to start", self.container)),
        }
    }


    async fn stop(&self) -> Result<(), String> {
        let container = self.docker.containers().get(&self.container);

        let stop_opts = ContainerStopOpts::builder().build();

        match container.stop(&stop_opts).await {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Container {} failed to stop", self.container)),
        }
    }
}
