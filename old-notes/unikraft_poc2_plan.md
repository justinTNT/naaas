# Unikraft POC 2.0 Plan

## Executive Summary

After discovering that Hermit requires KVM (unavailable on standard EC2), we're pivoting to Unikraft with Xen hypervisor. Unikraft is a Linux Foundation/Xen Project that supports standard cloud infrastructure without nested virtualization requirements.

## Key Advantages Over Hermit

1. **Cloud Native**: Official AWS platform support via `unikraft/plat-aws`
2. **Hypervisor Flexibility**: Supports KVM, Xen, Solo5, and bare metal
3. **Proven Performance**: 50% efficiency improvements over standard Linux on EC2
4. **Rust/Tokio Support**: Native support for Rust HTTP servers with Tokio/Hyper stack
5. **POSIX Compliance**: 130+ syscalls supported with mainstream language compatibility

## Technology Stack

- **Framework**: Unikraft (Linux Foundation/Xen Project)
- **Hypervisor**: Xen (AWS compatible)
- **Language**: Rust
- **HTTP Stack**: Tokio + Hyper
- **Cloud Platform**: AWS EC2 (standard instances)
- **Deployment**: Xen target images via `plat-aws`

## POC 2.0 Goals

Create a "Hello, Hyper!" HTTP server unikernel that:
1. Builds locally with Unikraft toolchain
2. Deploys to standard AWS EC2 instances
3. Serves HTTP requests with Tokio/Hyper
4. Demonstrates cloud-native unikernel deployment

## Implementation Steps

### Phase 1: Local Setup
1. Install Unikraft build system and dependencies
2. Set up Rust application template with Tokio/Hyper
3. Configure Kraftfile for Xen target
4. Build and test locally

### Phase 2: AWS Deployment
1. Generate Xen target image
2. Deploy using `unikraft/plat-aws` scripts
3. Configure EC2 security groups and networking
4. Test HTTP connectivity from external clients

### Phase 3: Validation
1. Verify "Hello, Hyper!" response via curl
2. Measure performance characteristics
3. Document deployment process
4. Compare with traditional container deployment

## Success Criteria

- [ ] Successful local build of Unikraft unikernel
- [ ] HTTP server responds with "Hello, Hyper!" message
- [ ] Deployment to standard AWS EC2 instances (no bare metal required)
- [ ] External HTTP connectivity validation
- [ ] Performance measurement and documentation

## Risk Mitigation

Unlike Hermit, Unikraft:
- Has official AWS deployment tools
- Doesn't require nested virtualization
- Has proven cloud performance track record
- Maintains active community and documentation

## Next Steps

1. Install Unikraft development environment
2. Create basic HTTP server application
3. Generate Xen target for AWS deployment
4. Validate end-to-end workflow

This approach should resolve the KVM dependency issue while maintaining unikernel benefits.