# Implementation Plan

> This is an early sketch of the roadmap. Nothing is certain in this plan.

## Chapter I: Core Engine
- [ ] Entity Viewer
  - [ ] List
  - [ ] View
  - [ ] Create
  - [ ] Patch
  - [ ] Remove
- [ ] Component Viewer
  - [ ] List
  - [ ] View
  - [ ] Create
  - [ ] Patch Name
  - [ ] Patch Fields
    - [ ] List
    - [ ] Create
    - [ ] Edit
    - [ ] Remove
  - [ ] Remove
  - [ ] Component Types
    - [ ] String
    - [ ] Integer
    - [ ] Boolean
    - [ ] 
- [ ] Storage
  - [ ] Table `Entity (UUID entity, INT created_at)`
  - [ ] Table `Component (UUID component, STRING name, INT created_at, BOOL is_system)`
  - [ ] Table `ComponentFields(UUID component, STRING field_name, STRING field_type, BOOL is_system)`
  - [ ] Table `EntityComponent(UUID entity, UUID component)`
  - [ ] Component Table `<Component> (UUID entity, <...>)`
- [ ] Admin UI
  - [ ] List Entities
  - [ ] View Entities
  - [ ] Create Entities
  - [ ] Patch Entities
  - [ ] Remove Entities
  - [x] List Components
  - [ ] View Components
  - [ ] Create Components
  - [ ] Patch Components
  - [ ] Remove Components

## Chapter II: Query API
- [ ] Pagination
- [ ] Query Filters
  - [ ] `query User, EmailAuth(email)`
  - [ ] function: `any (User, Superuser)`
  - [ ] function: `not (Superuser)`
  - [ ] `where email = {}`
- [ ] Component Restraints
  - [ ] Unique Fields (username)
  - [ ] Regular Expression String Validation
  - [ ] Integer Range

## Chapter III: Simple Authentication
- [ ] `User`
- [ ] `Superuser`
- [ ] `PassAuth`

## Chapter IV: Github CI and Benchmarking
Introduce CI to benchmark pull request against current standard speed and start profiling.

## Chapter V: Demo
Since this project was inspired

## Chapter VI: File Storage
- [ ] Component Type: 'File'
  - Restriction by Extension `jpg`
  - Restriction by Type `img`

## Chapter: Archetypes
We will have a lot of tables and table joins for longer queries so it makes sense to 'pack' queries into prototypes with component hierarchies. For example `User` and `PassAuth` occur more often together than `User` and `Message`, so for performance 'archetype' tables could be made: Reduces SQL joins needed for a query.

Either solve this algorithmatically or force superusers to generate such "indeces" or cache (or call them chunks if you want)

## Chapter: Extended Authentication
- [ ] `OTPAuth`
- [ ] `OAuth`

## Chapter: Firewall Filter
- [ ] Use existing filters as firewall to ... entities.
  - [ ] List
  - [ ] View
  - [ ] Create
  - [ ] Patch
  - [ ] Remove

## Chapter: Postgres Support
- [ ] Add features `postgres` and `sqlight` and allow library user to swap.

## Chapter: Realtime
- [ ] WebSockets: Subscriptions to Query (Track modifications)
  - [ ] List
  - [ ] View
  - [ ] Create
  - [ ] Patch
  - [ ] Remove
- [ ] Think about redis implementation

## Scaling
- [ ] Vertical
  - [ ] Connection Pooling
  - [ ] Query Optimization
- [ ] Postgres
- [ ] Horizontal
  - [ ] Stateless app servers
  - [ ] Shared DB
  - [ ] Redis