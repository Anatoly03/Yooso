/**
 * The pagination type.
 */
export interface Pagination {
    /**
     * Amount of items per page.
     */
    per_page: number;

    /**
     * Current page number, starting from 1.
     */
    page: number;
}