# YAM Repo Workflow Brief

This file gives the external ChatGPT project just enough repository-process
context to stay helpful without turning the source pack into a second workflow
manual.

## Authority Chain

- the real YAM repository is authoritative
- YAM docs inside the repo are authoritative planning and contract surfaces
- this source pack is a derived export for external ChatGPT context
- ChatGPT output is candidate material only until it is promoted back into the
  repo docs

## Repo Posture For 0.4

The current repo posture is conservative:

- `main` is the stable baseline
- verification should stay green
- docs, audit, and backlog should stay aligned
- greenhouse work should begin as pure contract/data prep before visible
  runtime behavior expands

## What ChatGPT Should Assume About Git Hygiene

ChatGPT should assume:

- meaningful planning should map back to a reviewable batch
- derived export packs can drift and therefore need provenance
- readiness gates matter more than optimistic roadmaps
- the project prefers deliberate sequencing over speculative branching trees

ChatGPT should not assume:

- the upload pack is a full repo mirror
- every future branch/tag/milestone policy must be restated inside the pack
- missing implementation means missing direction

## Helpful Kinds Of Workflow Advice

Useful external advice:

- suggest crisp phase boundaries
- suggest compact checkpoint names
- suggest doc-promotion order
- call out places where source-pack context may go stale
- distinguish immediate next work from later optional work

Less useful external advice:

- generic Git tutorials
- large branch taxonomies detached from actual YAM work
- governance prose that duplicates the repo docs
- pretending the pack alone is enough to audit implementation state

## Preferred ChatGPT Stance

The best external ChatGPT stance for YAM 0.4 is:

- bounded
- contract-aware
- option-oriented
- suspicious of scope creep
- explicit about assumptions
