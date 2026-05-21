# Yōso API

Yōso API builts onto the [Rocket](https://rocket.rs/) framework to provide an HTTP layer to interact with the entity component operations.

## API Reference

<!-- Section Component -->

#### `GET /api/components`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `GET /api/components/<uuid>`

<!-- ```ts
{
    metadata: {
        id: UUID;
        name: string;
        is_system: boolean;
        color: number; // in format RGB0
        created_at: number; // unix time stamp
    },
    fields: {
        id: UUID;
        name: string;
        field_type: "text" | "number" | "boolean";
        is_system: boolean;
        created_at: number;
    }[]
}
``` -->

- `200 OK`: Returns a JSON object of the component, metadata and field metadata.
- `400 Bad Request`: The provided UUID could not be parsed.
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
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

<!-- ```ts
{
    id: UUID;
    created_at: number; // unix time stamp
}
``` -->

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

#### `POST /api/entities/<uuid>/components/<uuid>`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->

#### `PATCH /api/entities/<uuid>/components/<uuid>`
- TODO
<!-- - `200 OK`: TODO -->
<!-- - `201 Created`: TODO -->
<!-- - `400 Bad Request`: TODO -->
<!-- - `401 Unauthorized`: TODO -->
<!-- - `403 Forbidden`: TODO -->
<!-- - `404 Not Found`: TODO -->
