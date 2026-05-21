/**
 * @file api.ts
 */

import { Pagination } from './pagination';
import Yooso from './yooso';

/**
 * The Yooso entity manager is created from Yooso and provides methods to
 * interact with entity management.
 */
export default class YoosoEntityManager {
    /**
     * Subscribed references.
     */
    #loadingRefs: { value: boolean }[] = [];

    /**
     * Subscribed error references.
     */
    #errorRefs: { value: string | null }[] = [];

    /**
     * Creates a new Yooso entity manager.
     * @param yooso The Yooso instance.
     */
    public constructor(private yooso: Yooso) {}

    /**
     * Subscribes a loading state reference to the entity manager. This
     * variable will be set to `true` when a request is in progress and `false`
     * when it is finished.
     */
    public subscribeLoadingRef(ref: { value: boolean }): this {
        this.#loadingRefs.push(ref);
        return this;
    }

    /**
     * Subscribes an error reference to the entity manager. This variable will
     * be set to the error message when an error occurs during a request.
     */
    public subscribeErrorRef(ref: { value: string | null }): this {
        this.#errorRefs.push(ref);
        return this;
    }

    /**
     * @param loading Loading state
     */
    private setLoading(loading: boolean) {
        for (const ref of this.#loadingRefs) {
            ref.value = loading;
        }
    }

    /**
     * @param error Error state
     */
    private setError(error: string | null) {
        for (const ref of this.#errorRefs) {
            ref.value = error;
        }
    }

    /**
     * Lists the available entities on the Yooso server. If an error occurs,
     * returns the empty array.
     * @param options Optional parameters for pagination. `per_page` is the number of components to display
     *   per page, and `page` is the page number to return (starting from 1). If provided, these parameters
     *   will be mutated to reflect response values.
     */
    public async list(options?: Pagination): Promise<any[]> {
        try {
            this.setLoading(true);
            const response = await this.yooso.get('/api/entities/list', { query: options });

            if (!response.ok) {
                const text = await response.text();
                this.setError(text || response.statusText);
            }

            const result = await response.json();
            this.setLoading(false);

            if (!result.success) {
                this.setError(result.message || 'Failed to fetch entities');
                return [];
            }

            this.setError(null);
            return result.entities;
        } catch (e) {
            this.setLoading(false);
            this.setError((e as Error).message || 'An unknown error occurred while fetching entities');
            return [];
        }
    }

    /**
     * Creates a new entity on the Yooso server.
     */
    public async create(): Promise<void> {
        try {
            this.setLoading(true);
            const response = await this.yooso.post('/api/entities');
            this.setLoading(false);

            if (!response.ok) {
                this.setError('Failed to create entity');
                return;
            }

            this.setError(null);
        } catch (e) {
            this.setLoading(false);
            this.setError((e as Error).message || 'An unknown error occurred while creating an entity');
        }
    }

    // TODO view entity

    // TODO PATCH entity components

    /**
     * Deletes an entity by its UUID. If an error occurs, returns false.
     */
    public async delete(uuid: string): Promise<boolean> {
        try {
            this.setLoading(true);
            const response = await this.yooso.delete(`/api/entities/${uuid}`);
            this.setLoading(false);

            if (!response.ok) {
                this.setError('Failed to delete entity');
                return false;
            }

            this.setError(null);
            return true;
        } catch (e) {
            this.setLoading(false);
            this.setError((e as Error).message || 'An unknown error occurred while deleting an entity');
            return false;
        }
    }
}
