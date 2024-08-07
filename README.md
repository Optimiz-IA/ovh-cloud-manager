# OVH Cloud Manager

Wrapper for OVH cloud API. For now, designed to interact with the AI Training part of cloud services.

Rust structures to fit with API form/response objects.

## Manager's credentials

Create a token here : <https://www.ovh.com/auth/api/createToken?GET=/cloud/*&PUT=/cloud/*&POST=/cloud/*&DELETE=/cloud/*>

### List of OVH regions

- ovh-eu
- ovh-us
- ovh-ca
- kimsufi-eu
- kimsufi-ca
- soyoustart-eu
- soyoustart-ca

### Environment variables for testing

- OVH_REGION="ovh-eu"
- OVH_APPLICATION_KEY="XXXXX"
- OVH_APPLICATION_SECRET="XXXXX"
- OVH_CONSUMER_KEY="XXXXX"
- OVH_PROJECT_ID="XXXXX"

## Manager methods

### Capabilities

- get_ai_quota
- list_ai_flavors
- get_ai_flavor_details

### Data

- list_ai_aliases
- get_ai_alias
- create_ai_alias
- update_ai_alias
- delete_ai_alias

### Jobs

- list_ai_jobs
- get_ai_job
- create_ai_job
- delete_ai_job
