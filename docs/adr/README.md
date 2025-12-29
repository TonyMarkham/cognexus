# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records (ADRs) for the Cognexus project.

## What is an ADR?

An Architecture Decision Record (ADR) is a document that captures an important architectural decision made along with its context and consequences.

**Purpose:**
- Document why decisions were made (context)
- Record what alternatives were considered (transparency)
- Track consequences and trade-offs (learning)
- Provide historical context for future maintainers

**When to write an ADR:**
- Significant architectural decisions (camera systems, rendering pipelines, data models)
- Technology choices (libraries, frameworks, protocols)
- Design patterns that will be used throughout the codebase
- Trade-offs between competing concerns (performance vs simplicity, etc.)

**When NOT to write an ADR:**
- Implementation details that don't affect architecture
- Bug fixes
- Routine feature additions that follow existing patterns
- Coding style preferences (those go in AGENTS.md)

## ADR Lifecycle

1. **Proposed** - Draft stage, open for discussion
2. **Accepted** - Decision made, ready to implement
3. **Deprecated** - No longer current, but kept for historical context
4. **Superseded** - Replaced by a newer ADR (link to it)

**Important:** ADRs are never deleted. They provide an audit trail of architectural evolution.

## Format

Use `template.md` as a starting point. Key sections:

- **Context** - Why are we making this decision?
- **Decision** - What are we doing?
- **Consequences** - What happens as a result?
- **Alternatives** - What else did we consider?

Keep ADRs concise but complete. Aim for 2-4 pages max.

## Numbering

ADRs are numbered sequentially: `0001-title.md`, `0002-title.md`, etc.

The number is permanent and never changes, even if the ADR is superseded.

## Writing Style

- Use clear, direct language
- Assume the reader is a future maintainer (could be you in 6 months)
- Include enough context that someone unfamiliar with the decision can understand it
- Focus on "why" more than "how" (implementation guides go in `docs/dev/`)
- Be honest about trade-offs and downsides

## Current ADRs

- [ADR-0001: Camera and Grid Rendering System](0001-camera-and-grid-system.md) - Proposed
- [ADR-0002: Camera Control Input System](0002-camera-control-input-system.md) - Proposed
- [ADR-0003: GPU-Accelerated Visual Effects and Rendering](0003-gpu-accelerated-visual-effects-and-rendering.md) - Accepted

## Resources

- [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) - Original ADR article by Michael Nygard
- [ADR GitHub Organization](https://adr.github.io/) - Tools and examples
- [When to Use ADRs](https://github.com/joelparkerhenderson/architecture-decision-record#when-should-we-write-adrs) - Practical guidance
