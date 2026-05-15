/**
 * @file api.ts
 */

import YoosoComponentManager from './components';

/**
 * The Yooso instance communicates with a Yooso server and provides bindings to
 * interact with the server API.
 */
export default class Yooso {
    /**
     * Indicates whether the connection to the Yooso server is secure (HTTPS)
     * or not (HTTP). Also relevant for WebSocket connections (WSS vs WS).
     */
    private secure: boolean = null!;

    /**
     * The hostname of the server, trimmed of the protocol and path.
     * Resulting values are for example `yooso.com` or `localhost:8080`.
     */
    private hostname: string = null!;

    /**
     * Creates a new Yooso instance.
     * @param url The URL of the Yooso server.
     */
    public constructor(url: string) {
        // Remove protocol from URL and set secure flag
        if (url.startsWith('http://')) {
            this.secure = false;
            url = url.substring(7);
        } else if (url.startsWith('https://')) {
            this.secure = true;
            url = url.substring(8);
        }

        // Remove path (everything after first flash)
        let slashIndex = url.indexOf('/');
        if (slashIndex !== -1) {
            url = url.substring(0, slashIndex);
        }

        // For special "local" hostnames, if the secure flag is not set,
        // automatically set secure flag.
        if (this.secure === null) {
            this.secure = !(url === 'localhost' || url.startsWith('localhost:') || url === '127.0.0.1');
        }

        this.hostname = url;
    }

    /**
     * @returns The host URL, including protocol, but without path. For example,
     * `https://yooso.com` or `http://localhost:8080`.
     */
    public get host(): string {
        return (this.secure ? 'https://' : 'http://') + this.hostname;
    }

    /**
     * Fetches a resource from the Yooso server. Wraps around {@link fetch} and
     * prepends the host URL to the path.
     *
     * @param path The path to the resource, starting with a slash.
     * @param options The options to pass to {@link fetch}.
     */
    private async fetch(path: `/${string}`, options?: RequestInit): Promise<Response> {
        const response = await fetch(this.host + path, options);

        if (!response.ok) {
            console.error(`GET yooso/${path} -> ${response.status}: ${response.statusText}`);
        }

        return response;
    }

    /**
     * Fetches a resource from the Yooso server.
     * @param path The path to the resource, starting with a slash.
     * @returns A promise resolving to the response.
     */
    public get(path: `/${string}`): Promise<Response> {
        return this.fetch(path, { method: 'GET' });
    }

    /**
     * Posts a JSON resource to the Yooso server.
     * @param path The path to the resource, starting with a slash.
     * @param body The body of the request, an object that will be stringified to JSON.
     * @returns A promise resolving to the response.
     *
     * ### Headers
     *
     * ```yaml
     * Content-Type: application/json
     * ```
     */
    public post(path: `/${string}`, body: object): Promise<Response>;

    /**
     * Posts a resource to the Yooso server.
     * @param path The path to the resource, starting with a slash.
     * @param body The body of the request, a plain string.
     * @returns A promise resolving to the response.
     *
     * ### Headers
     *
     * ```yaml
     * Content-Type: text/plain
     * ```
     */
    public post(path: `/${string}`, body: RequestInit['body']): Promise<Response>;

    // implementation
    public post(path: `/${string}`, body: any): Promise<Response> {
        return this.fetch(path, {
            method: 'POST',
            headers: {
                'Content-Type': typeof body === 'string' ? 'text/plain' : 'application/json',
            },
            body: typeof body === 'string' ? body : JSON.stringify(body),
        });
    }

    /**
     * Patches a JSON resource to the Yooso server.
     * @param path The path to the resource, starting with a slash.
     * @param body The body of the request, an object that will be stringified to JSON.
     * @returns A promise resolving to the response.
     *
     * ### Headers
     *
     * ```yaml
     * Content-Type: application/json
     * ```
     */
    public patch(path: `/${string}`, body: object): Promise<Response>;

    /**
     * Patches a resource on the Yooso server.
     * @param path The path to the resource, starting with a slash.
     * @param body The body of the request, a plain string.
     * @returns A promise resolving to the response.
     *
     * ### Headers
     *
     * ```yaml
     * Content-Type: text/plain
     * ```
     */
    public patch(path: `/${string}`, body: RequestInit['body']): Promise<Response>;

    // implementation
    public patch(path: `/${string}`, body: any): Promise<Response> {
        return this.fetch(path, {
            method: 'PATCH',
            headers: {
                'Content-Type': typeof body === 'string' ? 'text/plain' : 'application/json',
            },
            body: typeof body === 'string' ? body : JSON.stringify(body),
        });
    }

    /**
     * Deletes a resource on the Yooso server.
     * @param path The path to the resource, starting with a slash.
     * @returns A promise resolving to the response.
     */
    public delete(path: `/${string}`): Promise<Response> {
        return this.fetch(path, { method: 'DELETE' });
    }

    /**
     * Creates a new Yooso component manager.
     * @returns {YoosoComponentManager}
     */
    public components() {
        return new YoosoComponentManager(this);
    }
}
