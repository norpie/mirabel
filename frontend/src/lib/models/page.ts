export interface Page {
    page: number;
    pageSize: number;
}

export interface PageResponse<T> {
    data: T;
    pageInfo: Page;
}
