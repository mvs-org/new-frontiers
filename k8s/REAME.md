
# important

This directory is the directory of the yaml files needed to deploy k8s. The following points need to be paid attention to

1. The file path specified by hostPath must be an absolute path
2. The "validator/deployment-validator.yaml" file bootnodes parameter needs to be specified as the clusterIp address of the boot-node
3. "boot-node" must be deployed first
