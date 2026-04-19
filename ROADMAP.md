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
  - [x] List
  - [ ] View
  - [x] Create
  - [ ] Patch Name
  - [ ] Patch Fields
    - [ ] List
    - [ ] Create
    - [ ] Edit
    - [ ] Remove
  - [x] Remove
  - [ ] Component Types
    - [ ] String
    - [ ] Integer
    - [ ] Boolean
    - [ ] 
- [x] Storage
  - [x] Table `Entity (UUID entity, INT created_at)`
  - [x] Table `Component (UUID component, STRING name, INT created_at, BOOL is_system)`
  - [x] Table `ComponentFields(UUID component, STRING field_name, STRING field_type, BOOL is_system)`
  <!-- - [ ] Table `EntityComponent(UUID entity, UUID component)` -->
  - [ ] Component Table `<Component> (UUID entity, <...>)`

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

## Chapter VI: Powerful Admin Panel
The home page in the admin UI should be customizable. Boxes counting and listing some queries to provide statistics. For example: "Users: 100", "Messages: 567", etc.

## Chapter VII: File Storage
- [ ] Component Type: 'File'
  - Restriction by Extension `jpg`
  - Restriction by Type `img`

## Chapter: Memory Components
Components per default are 'stored', however there should exist 'memory' components which unlocks streaming-like field types. These are useful for data like currently-online users (erased after server restarts), or opened 'rooms' (useful in online video games) and streaming channels.

The cool thing is that if we support the admin UI to have customizable dashboard counting components per entities things like "Online Users: 25" are automatically possible.

- [ ] Memory Components
  - [ ] Audio Streaming Channel
  - [ ] Video Streaming Channel

## Chapter: Archetypes
We will have a lot of tables and table joins for longer queries so it makes sense to 'pack' queries into prototypes with component hierarchies. For example `User` and `PassAuth` occur more often together than `User` and `Message`, so for performance 'archetype' tables could be made: Reduces SQL joins needed for a query.

Either solve this algorithmatically or force superusers to generate such "indeces" or cache (or call them chunks if you want)

Also SQL Indeces: https://sqlite.org/partialindex.html

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
- [ ] Add features `postgres` and `sqlite` and allow library user to swap.

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