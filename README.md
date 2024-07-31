# My Awesome Rust Microservices on Kubernetes

Welcome to my super cool project where I deploy Rust microservices on a Kubernetes cluster. This repo includes all the Kubernetes configs, Dockerfiles, and Helm charts you'll need to get things running smoothly.

## What’s Inside?

- **Rust Microservices**: Built with the awesome `actix-web` framework.
- **Dockerfiles**: To containerize the microservices.
- **Kubernetes Manifests**: For deploying the services, ingress, and configurations.
- **Helm Charts**: To make deployment a breeze.

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [Docker](https://www.docker.com/get-started)
- [Kubernetes](https://kubernetes.io/docs/setup/) (minikube or a full cluster)
- [Helm](https://helm.sh/docs/intro/install/)

### Cloning the Repo

First things first, clone this repo to your local machine:

```bash
git clone https://github.com/yourusername/awesome-rust-k8s.git
cd awesome-rust-k8s



### Building the Docker Images

Navigate to the directory of each microservice and build the Docker image:

```bash
cd user-service
docker build -t your-dockerhub-username/user-service:latest .
```

Repeat for any other microservices.

### Pushing Docker Images

Push the images to your Docker Hub account:

```bash
docker push your-dockerhub-username/user-service:latest
```

### Deploying to Kubernetes

First, make sure your Kubernetes cluster is up and running. Then, deploy the services using Helm:

```bash
helm install myapp ./myapp
```

This will deploy your microservices along with all the necessary configurations.

### Updating the Deployment

Made some changes? No problem! Just update your deployment with:

```bash
helm upgrade myapp ./myapp
```

## Exploring the Setup

- **Check Deployments**: `kubectl get deployments`
- **Check Services**: `kubectl get services`
- **Check Pods**: `kubectl get pods`

### Accessing the Application

Assuming you’ve configured ingress correctly, access your services via the host you specified (e.g., `http://example.com/user`).

## Configuration

You can tweak the configurations in the `values.yaml` file under the Helm chart directory:

```yaml
replicaCount: 2

image:
  repository: your-dockerhub-username/user-service
  pullPolicy: IfNotPresent
  tag: "latest"

service:
  type: ClusterIP
  port: 80

ingress:
  enabled: true
  annotations: {}
  hosts:
    - host: example.com
      paths:
        - path: /user
          pathType: Prefix

config:
  APP_ENV: "production"
  APP_SECRET: "supersecretkey"
```

## Contributing

Found a bug or have an idea? Feel free to open an issue or submit a pull request. Let’s make this project even more awesome together!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Happy coding! 
