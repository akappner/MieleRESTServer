# Home Assistant add-on setup

First, ensure you did **step 2 Provision cryptographic keys** from [`Standalone setup`](standalone-setup.md)  

## Add repository and install

1. In Home Assistant, open `Settings -> Add-ons -> Add-on Store -> menu -> Repositories`.
2. Add this repository URL.
3. Go ack to your addons, search for `Miele REST Server`.
4. It should now show the addon, install it.

## Configure the add-on

set the endpoint values in the HA UI under add-on options (`endpoints`).

#### Compatibility mode
This repo used to use a config file. To stay backwards compatible, we still support this.
However it's recommended to use the addon options to configure the MieleRESTServer instead.  
if `endpoints` is empty, create `/config/MieleRESTServer.config`  
Template: `examples/MieleRESTServer-example-config.yaml`

#### Optional Home Assistant REST sensor integration
Use `examples/homeassistant-configuration-fragment.yaml`

