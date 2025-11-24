# NAAAS: Not-Another-as-a-Service

> Thin infrastructure wrapper for monolithic applications using unikernels

## Current Status: Sprint 1 Ready 🚀

✅ **Technology Proven**: Unikraft + Rust + Tokio/Hyper working on AWS  
✅ **Production Validated**: HTTP server live at http://3.1.210.183:3000  
✅ **Documentation Complete**: Full project vision and roadmap  
🚧 **Next**: Build `naaas-server` control plane (Sprint 1)

## Quick Orientation

- **`project.md`** - Core vision: "thin infrastructure wrapper" philosophy
- **`notes/roadmap.md`** - Sprint-based development plan  
- **`notes/naaas_requirements.md`** - Technical specs for all components
- **`unikraft-hyper-poc/`** - Working HTTP server unikernel (production tested)
- **`*_poc*.md`** - POC journey (Hermit→Unikraft pivot, results)

## What NAAAS Does

Wraps monolithic apps (like Ghost CMS) with per-tenant unikernel shims that provide:
- 🔐 TLS termination  
- 🚦 Rate limiting
- 📊 Access logging  
- 🏷️ Config serving

**NOT**: Data isolation, app logic, business features - that stays in your monolith.

## Development Phases

- ✅ **Phase 0**: Unikernel technology validation  
- 🚧 **Sprint 1**: `naaas-server` control plane + `naaas-ctl` CLI
- ⏳ **Sprint 2**: `naaas-shim` transparent proxy unikernel  
- ⏳ **Sprint 3**: Infrastructure features (TLS, rate limiting, logging)
- ⏳ **Sprint 4**: Dashboard + AWS AMI packaging

## Git Repository

- **Committed**: Core docs, working unikernel code, POC results
- **Ignored**: Build artifacts, external tools, secrets  
- **Current branch**: `main` (ready for Sprint 1 development)

Quick start: `git log --oneline` to see what we've built so far.
