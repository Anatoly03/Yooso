/**
 * @file api.ts
 */

import Yooso from './yooso';

/**
 * Represents a component in the Yooso system.
 */
export type Component = {
    /**
     * The unique identifier of the component. This is a UUID string.
     */
    id: string;
    /**
     * The name of the component. This is a string that must be a valid SQL identifier and not a SQL keyword.
     */
    component_name: string;
    /**
     * The color associated with the component.
     */
    color: number;
    /**
     * Indicates whether the component is a system component.
     */
    is_system: boolean;
    /**
     * The timestamp when the component was created, in milliseconds since the Unix epoch.
     */
    created_at: number;
};

export type CreateComponentRequest = {
    /**
     * The name of the component. This is a string that must be a valid SQL identifier and not a SQL keyword.
     */
    name: string;
    /**
     * The color associated with the component.
     */
    color: number;
    /**
     * Indicates whether the component is a system component.
     */
    is_system: boolean;
    /**
     * Array of fields in the component.
     */
    fields: {
        /**
         * The name of the field. This is a string that must be a valid SQL identifier and not a SQL keyword.
         */
        name: string;
        /**
         * The type of the field.
         */
        field_type: string;
        /**
         * Indicates whether the field is a system field.
         */
        is_system: boolean;
    }[];
};

/**
 * The Yooso component manager is created from Yooso and provides methods to
 * interact with component management.
 */
export default class YoosoComponentManager {
    /**
     * Subscribed references.
     */
    #loadingRefs: { value: boolean }[] = [];

    /**
     * Subscribed error references.
     */
    #errorRefs: { value: string | null }[] = [];

    /**
     * Creates a new Yooso component manager.
     * @param yooso The Yooso instance.
     */
    public constructor(private yooso: Yooso) {}

    /**
     * Subscribes a loading state reference to the component manager. This
     * variable will be set to `true` when a request is in progress and `false`
     * when it is finished.
     */
    public subscribeLoadingRef(ref: { value: boolean }): this {
        this.#loadingRefs.push(ref);
        return this;
    }

    /**
     * Subscribes an error reference to the component manager. This variable will
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
     * Lists the available components on the Yooso server. If an error occurs,
     * returns the empty array.
     */
    public async list(): Promise<Component[]> {
        try {
            this.setLoading(true);
            const response = await this.yooso.get('/api/components/list');
            const result = await response.json();
            this.setLoading(false);

            if (!result.success) {
                this.setError(result.message || 'Failed to fetch components');
                return [];
            }

            this.setError(null);
            return result.components;
        } catch (e) {
            this.setLoading(false);
            this.setError((e as Error).message || 'An unknown error occurred while fetching components');
            return [];
        }
    }

    /**
     * Creates a new component on the Yooso server.
     */
    public async create(component: CreateComponentRequest): Promise<void> {
        try {
            this.setLoading(true);
            const response = await this.yooso.post('/api/components', component);
            const result = await response.json();
            this.setLoading(false);

            if (!result.success) {
                this.setError(result.message || 'Failed to create component');
                return;
            }

            this.setError(null);
        } catch (e) {
            this.setLoading(false);
            this.setError((e as Error).message || 'An unknown error occurred while creating a component');
        }
    }
}
