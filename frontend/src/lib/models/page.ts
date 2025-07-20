export interface PageInfo {
    page: number;
    size: number;
    total: number;
}

export interface PageResponse<T> {
    data: T;
    pageInfo: PageInfo;
}

export interface CursorPageResponse<T> {
    data: T;
    hasMore: boolean;
    nextCursor?: string;
    prevCursor?: string;
}
