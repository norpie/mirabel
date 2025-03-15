export interface Page {
    page: number;
    size: number;
}

export interface PageResponse<T> {
    data: T;
    pageInfo: Page;
}
