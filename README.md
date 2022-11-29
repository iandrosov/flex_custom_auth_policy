# Flex Gateway custom authenticatio policy

## Overview
Example custom authentication policy for MuleSoft Flex Gateway using Rust SDK.

Flex Gateway is a new addition to the Anypoint platform released in spring of 2022, to enable customers manage any API anywhere. Anyone who has faced a challenge of managing APIs at scale in the enterprise would agree that one critical aspect is security and governance for APIs. The Anypoint platform offers many features to protect APIs in the form of standard policies to support NON-Functional requirements (NFR). It is important for customers to build their own custom policies to fit unique requirements. For more on this topic will be explored in this blog article: [Develop custom policy for Flex Gateway](TBD)

The Rust project contains policy code implementation and configuration files related to Flex Gateway in connected and local mode.

## Configuration
### Connected mode
Connected mode related configuratiopn files are located
in `config/connected` directory. There are 3 files required to publish this policy to MuelSoft Anypoint exchange and apply policy via API Manager to API asset managed by Flex Gateway in connected mode:

```
flex_custom_auth_policy_schema.json
flex_custom_auth_policy_definition.yaml
flex_custom_auth_policy_metadata.yaml
```

### Local mode
Local mode related configuratiopn files are located
in `config/local` directory. There are several configuration files required to deploy this policy to MuelSoft Flex Gateway in local mode. For the purpose of demonstration we provide configurations in both Resource and Inline modes:

```
flex-custom-auth-policy.yaml - Inline mode combined configuration


```