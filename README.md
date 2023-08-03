# DEPLOY

1. Generate SSH keypair for deploy `ssh-keygen`
2. Give it a distinctive name at the prompt `deploy_id_rsa`
3. Copy public key to the server via ssh.  It needs to go into the ~/.ssh/authorized_keys file
    ```
   ssh-copy-id -i ~/.ssh/deploy_id_rsa.pub root@5.78.82.67
   ```
4. Put the private key into Github Secret (copy paste)
5. Now our github workflow is properly configured