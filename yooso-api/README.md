# Yōso API

Yōso API builts onto the [Rocket](https://rocket.rs/) framework to provide an HTTP layer to interact with the entity component operations.

## API Reference

<!-- Section Component -->

<!-- #### `GET /api/components` -->

#### `GET /api/components/<uuid>`
- `200 OK`: Returns a JSON object of the component, metadata and field metadata.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

<!-- #### `POST /api/components` -->

#### `DELETE /api/components/<uuid>`
- `200 OK`: The component with the specified UUID was deleted.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
- `404 Not Found`: The component was either already removed or did not exist in the first place.

<!-- #### `PATCH /api/components/<uuid>` -->

<!-- Section Entities -->

<!-- #### GET /api/entities -->
<!-- #### GET /api/entities/<uuid> -->

#### `POST /api/entities`
- `201 Created`:  The entity was created. Returns a JSON representation of the entity metadata.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `402 Payment Required`: TODO -->
<!-- - `403 Forbidden`: TODO -->

#### `DELETE /api/entities/<uuid>`
- `200 OK`: The entity with the specified UUID was deleted and removed from all components.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
- `404 Not Found`: The entity was either already removed or did not exist in the first place.

<!-- #### POST /api/entities/<uuid>/components/<uuid> -->
<!-- #### PATCH /api/entities/<uuid>/components/<uuid> -->
