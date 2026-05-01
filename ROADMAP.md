# Implementation Plan (Sorted Timeline)

## Chapter I: Core Engine
- [x] Entity Viewer
  - [x] List (Entities)
  - [x] View (Components per Entity)
  - [x] Create (Entity)
  - [x] Remove (Entity)
    - [x] Batch Remove Related Components
  - [x] Patch (Components per Entity)
    - [x] Add (Components to Entity)
    - [x] View (Component Field Data of Entity)
    - [x] Edit (Components of Entity)
    - [x] Remove (Components of Entity)
- [x] Component Viewer
  - [x] List (Components)
  - [x] View (Component Fields and Meta)
  - [x] Create Component
    - [x] Create Fields
  - [x] Patch Name and Color
  - [x] Remove (Component)
  - [x] Component Fields
    - [x] List
    - [x] Create (Patch Component)
    - [ ] Edit Field Name (Patch Component, Field Type Migration not supported)
    - [x] Remove (Patch Component)
  - [x] Component Field Types
    - [x] String
    - [x] Integer
    - [x] Boolean
- [x] Storage
  - [x] Entity Table
  - [x] Component Metadata Table
  - [x] Component Fields Metadata Table
  - [x] (Generated) Component Tables

> This is an early sketch of the roadmap. Nothing is certain in this plan.

## Chapter II: Error Handling, Validation, Refactoring, Documentation

- [x] Add generic Error type
- [ ] Error Handling & Validation
  - [ ] Github CI
  - [ ] Entities
    - [ ] List (Entities)
    - [ ] View (Components per Entity)
    - [x] Create (Entity)
    - [ ] Remove (Entity)
    - [ ] Patch (Components per Entity)
      - [ ] Add (Components to Entity)
      - [ ] View (Component Field Data of Entity)
      - [ ] Edit (Components of Entity)
      - [ ] Remove (Components of Entity)
  - [ ] Components
    - [ ] List (Components)
    - [ ] View (Component Fields and Meta)
    - [ ] Create Component
      - [ ] Create Fields
    - [ ] Patch Name and Color
    - [ ] Remove (Component)
    - [ ] Component Fields
      - [ ] List
      - [ ] Create (Patch Component)
      - [ ] Edit Field Name (Patch Component)
      - [ ] Remove (Patch Component)
    - [ ] Invalid Component Field Type
  - [ ] Storage
    - [ ] Entity Table
    - [ ] Component Metadata Table
    - [ ] Component Fields Metadata Table
    - [ ] (Generated) Component Tables
  - [ ] `cargo fmt`
  - [ ] `cargo clippy`
- [ ] Refactor
  - [ ] Centralize Field Validation (allowed field types, supported field names, duplicate/ empty checks)
  - [ ] Move SQL construction behind tighter storage-facing helper (API handlers should not assembly them)
  - [ ] Remove all panics, replace with error propagation and HTTP responses
- [ ] Testing
  - [ ] Core Testing Library
  - [ ] Unit Testing
- [ ] Library Documentation
  - [x] Github CI
  - [ ] `yooso`
  - [ ] `yooso-api`
  - [ ] `yooso-core`
  - [x] `yooso-example`
  - [x] `yooso-macro`
  - [x] `yooso-storage`
- [ ] Client Documentation
  - [ ] Github CI
  - [ ] `yooso-studio`
- [ ] Refactor
  - [ ] Clean up `dash-case` in user interface and `snake_case` in sqlite.
  - [ ] Error Handling in Api.
  - [ ] Refactor Client networking to separate module.
    - [ ] Error Handling in Client.
    - [ ] Add 'loading' indicator to everything
  - [ ] Use Naive UI messages for notification communication bottom right.
    - https://www.naiveui.com/en-US/os-theme/components/message

## Chapter III: Query API
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
- [ ] Unit Tests

## Chapter IV: Simple Authentication
- [ ] `User`
- [ ] `Superuser`
- [ ] `PassAuth`
- [ ] `Verified`
- [ ] Security Analytics Tests
  - [ ] Only authenticated superusers can call component endpoints and edit entities.
  - [ ] Users can only read entities existing and attached components.

## Chapter V: Github CI and Benchmarking
Introduce CI to benchmark pull request against current standard speed and start profiling.

## Chapter VI: Demo
Since this project was inspired

## Chapter VII: Powerful Admin Panel
The home page in the admin UI should be customizable. Boxes counting and listing some queries to provide statistics. For example: "Users: 100", "Messages: 567", etc.

- [ ] Themes
  - https://www.naiveui.com/en-US/os-theme/docs/customize-theme
- [ ] Localization
  - https://vue-i18n.intlify.dev/guide/essentials/pluralization.html
  - [x] Technical Support for Localization (Ability to Select Locale)
  - [ ] Localize everything
    - [ ] Japanese
    - [ ] German
    - [ ] Chinese (use AI or find translators)

## Chapter VII: Firewall Filter
- [ ] Use existing filters as firewall to ... entities.
  - [ ] List
  - [ ] View
  - [ ] Create
  - [ ] Patch
  - [ ] Remove

# Future Chapters (Not Sorted)

## Chapter: File Storage
- [ ] Component Type: 'File'
  - Restriction by Extension `jpg`
  - Restriction by Type `img`

## Chapter: In-Memory Components
Components per default are 'stored', however there should exist 'memory' components which unlocks streaming-like field types. These are useful for data like currently-online users (erased after server restarts), or opened 'rooms' (useful in online video games) and streaming channels.

The cool thing is that if we support the admin UI to have customizable dashboard counting components per entities things like "Online Users: 25" are automatically possible.

- [ ] Memory Components
  - [ ] Audio Streaming Channel
  - [ ] Video Streaming Channel

## Chapter: Extend Built-In Authentication
- [ ] `OTPAuth`
- [ ] `OAuth`
- [ ] `2FA`

## Chapter: Postgres Support
- [ ] Add features `postgres` and `sqlite` and allow yooso user to swap.

## Chapter: Realtime
- [ ] WebSockets: Subscriptions to Query (Track modifications)
  - [ ] List
  - [ ] View
  - [ ] Create
  - [ ] Patch
  - [ ] Remove
- [ ] Think about redis implementation

## Chapter: Archetypes
We will have a lot of tables and table joins for longer queries so it makes sense to 'pack' queries into prototypes with component hierarchies. For example `User` and `PassAuth` occur more often together than `User` and `Message`, so for performance 'archetype' tables could be made: Reduces SQL joins needed for a query.

Either solve this algorithmatically or force superusers to generate such "indeces" or cache (or call them chunks if you want)

Also SQL Indeces: https://sqlite.org/partialindex.html

## Scaling
- [ ] Vertical
  - [ ] Connection Pooling
  - [ ] Query Optimization
- [ ] Postgres
- [ ] Horizontal
  - [ ] Stateless app servers
  - [ ] Shared DB
  - [ ] Redis