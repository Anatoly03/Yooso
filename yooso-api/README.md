# Yōso API

Yōso API builts onto the [Rocket](https://rocket.rs/) framework to provide an HTTP layer to interact with the entity component operations.

## API Reference

<!-- Section Component -->

<!-- Section Entities -->

<!-- GET /api/entities -->
<!-- GET /api/entities/<uuid> -->

##### `POST /api/entities`

Creates a new entity and returns its' UUID.

##### `DELETE /api/entities/<uuid>`

Removes the entity with the specified UUID recursively, which means it is detached from every component.

<!-- POST /api/entities/<uuid>/components/<uuid> -->
<!-- PATCH /api/entities/<uuid>/components/<uuid> -->
