import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  input: '../yooso-api/openapi/openapi.json',
  output: 'src/openapi',
});
