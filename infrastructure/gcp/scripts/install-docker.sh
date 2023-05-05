#! /bin/bash

DOCKER_NOT_FOUND=21
DOCKER_COMPOSE_NOT_FOUND=22

if [ "${is_boot_node}" == "TRUE" ]; then
  echo  BOOT_NODE="TRUE" >> /etc/profile
else
  echo  BOOT_NODE="FALSE" >> /etc/profile
fi

echo  KEY_PREFIX="${key_prefix}" >> /etc/profile
echo  BOOT_NODE_IP="${boot_node_ip}" >> /etc/profile

if [[ ! $(which docker) ]]; then
  echo "[$(date)] No docker found" >> deploy.log
  removeOld=$(sudo apt-get remove docker docker-engine docker.io containerd runc)
  echo "[$(date)] Output of removing old docker packages: $removeOld" >> deploy.log
  sudo apt-get update
  sudo apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
  dockerKey=$(sudo apt-key fingerprint 0EBFCD88)
  echo "[$(date)] Docker fingerprint: $dockerKey" >> deploy.log

  sudo add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   stable"

  sudo apt-get update
  sudo apt-get install -y docker-ce docker-ce-cli containerd.io

  echo "[$(date)] Docker after installation steps $(which docker)" >> deploy.log

  sudo usermod -a -G docker $USER
  sudo chmod 777 /var/run/docker.sock

  if [[ ! $(which docker) ]]; then
    exit $DOCKER_NOT_FOUND
  fi
fi

if [[ ! $(which docker-compose) ]]; then
  echo "[$(date)] No docker-compose found" >> deploy.log
  sudo curl -L "https://github.com/docker/compose/releases/download/1.27.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
  sudo chmod +x /usr/local/bin/docker-compose

  echo "[$(date)] Docker Compose after installation steps $(which docker-compose)" >> deploy.log
  if [[ ! $(which docker-compose) ]]; then
    exit $DOCKER_COMPOSE_NOT_FOUND
  fi
fi