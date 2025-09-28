export interface TableItem {
    operatorId: string;
    avsCount: number;
    strategyCount: number;
    slashingCount: number;
    lastSlashAt: number | null;
    lastUpdateBlockTs: number;
    tvlTotalAtomic: string;
    hhiStrategy: number;
    nonzeroStrategyCount: number;
}

export interface BarItem {
    operatorId: string;
    tvlTotalAtomic: string;
}

export interface DonutSlice {
    share: number;
    strategyId: string;
    tvlAtomic: string;
}

export interface DonutOperatorData {
    operatorId: string;
    slices: DonutSlice[];
}

export type Donut = Record<string, DonutOperatorData>;

export interface GraphItem {
    operatorId: string;
    strategyId: string;
    weightAtomic: string;
}

export interface Outliers {
    highConcentration: string[];
    zeroShare: string[];
    recentSlashes: string[];
}

export interface Meta {
    source: string;
    first: number;
    skip: number;
    count: number;
}

export interface ApiResponse {
    meta: Meta;
    table: TableItem[];
    bar: BarItem[];
    donut: Donut;
    graph: GraphItem[];
    outliers: Outliers;
}