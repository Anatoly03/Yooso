# Yōso API

Yōso API builts onto the [Rocket](https://rocket.rs/) framework to provide an HTTP layer to interact with the entity component operations. 

<!-- #### Convention Summary

- `200 OK`: Successful GET, PATCH requests and DELETEs.
- `201 Created`: Successful POSTs.
- `204 No Content`: DELETE operation called on already removed object.
- `400 Bad Request`: Invalid user input
- `401 Unauthorized`: TODO
- `403 Forbidden`: TODO
- `404 Not Found`: Either endpoints does not exist, or the relevant objects.
- `500 Internal Server Error`:  -->

## API Specification

<!-- Section Component -->

#### `GET /api/components`

- `200 OK`: Lists all visible components.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->

#### `GET /api/components/<uuid>`

- `200 OK`: Returns a JSON object of the component, metadata and field metadata.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `POST /api/components`

Initializes a new component in the Yooso application. Requires a JSON input body providing component metadata and fields metadata. Responds with a JSON object similar to the input, but with the attributes `id` and `created_at` set.

- `201 Created`: Returns a JSON object of the created component, metadata and field metadata.
- `400 Bad Request`: The request body could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `DELETE /api/components/<uuid>`

- `200 OK`: The component with the specified UUID was deleted.
- `204 No Content`: The component with the specified UUID was not found.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
- `404 Not Found`: The component was either already removed or did not exist in the first place.

#### `PATCH /api/components/<uuid>`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

<!-- Section Entities -->

#### `GET /api/entities`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `GET /api/entities/<uuid>`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `POST /api/entities`

- `201 Created`:  The entity was created. Returns a JSON representation of the entity metadata.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `402 Payment Required`: TODO -->
<!-- - `403 Forbidden`: TODO -->

#### `DELETE /api/entities/<uuid>`

- `200 OK`: The entity with the specified UUID was deleted and removed from all component tables.
- `204 No Content`: The component with the specified UUID was not found.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
- `404 Not Found`: The entity was either already removed or did not exist in the first place.

#### `POST /api/entities/<uuid>/components/<uuid>`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `204 No Content`: The component was not attached to the entity in the first place.  -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `DELETE /api/entities/<uuid>/components/<uuid>`
- `200 OK`: The component has been successfully detached from the entity.
- `204 No Content`: The component was not attached to the entity in the first place.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
- `404 Not Found`: The provided component does not exist.
