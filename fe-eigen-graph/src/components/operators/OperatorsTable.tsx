"use client";
import React, {useState, useMemo} from "react";
import {
    Table,
    TableBody,
    TableCell,
    TableHeader,
    TableRow,
} from "../ui/table";
import {TableItem} from "@/types/operators";
import {formatPowerOfTen} from "@/utils/number-utils";

interface OperatorsTableProps {
    tableData: TableItem[];
}

const shortenId = (id: string, chars = 6) => {
    if (id.length <= chars * 2 + 2) {
        return id;
    }
    return `${id.substring(0, chars + 2)}...${id.substring(id.length - chars)}`;
};

const formatTimestamp = (ts: number | null) => {
    if (ts === null) return "N/A";
    return new Date(ts * 1000).toLocaleDateString('en-US', {
        month: 'short',
        day: 'numeric',
        year: 'numeric',
    });
};

const SortArrow: React.FC<{ direction: 'asc' | 'desc', isActive: boolean }> = ({direction, isActive}) => (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg"
         className={`transition-colors ${isActive ? 'text-brand-500' : 'text-gray-400 dark:text-gray-600'}`}>
        {direction === 'asc' ? <path d="M6 4L3 8H9L6 4Z" fill="currentColor"/> :
            <path d="M6 8L9 4H3L6 8Z" fill="currentColor"/>}
    </svg>
);

const OperatorsTable: React.FC<OperatorsTableProps> = ({tableData}) => {
    const [sort, setSort] = useState<{ column: keyof TableItem | ""; asc: boolean }>({
        column: "tvlTotalAtomic",
        asc: false,
    });
    const [page, setPage] = useState<number>(1);
    const [searchTerm, setSearchTerm] = useState("");
    const perPage: number = 10;

    const headers: { key: keyof TableItem; label: string; sortable: boolean }[] = [
        {key: "operatorId", label: "Operator ID", sortable: false},
        {key: "tvlTotalAtomic", label: "TVL", sortable: true},
        {key: "avsCount", label: "AVSs", sortable: true},
        {key: "strategyCount", label: "Strategies", sortable: true},
        {key: "slashingCount", label: "Slashing Count", sortable: true},
        {key: "lastSlashAt", label: "Last Slash", sortable: true},
        {key: "lastUpdateBlockTs", label: "Last Update", sortable: true},
    ];

    const filteredAndSortedItems = useMemo(() => {
        const filtered = searchTerm
            ? tableData.filter(item => item.operatorId.toLowerCase().includes(searchTerm.toLowerCase()))
            : tableData;

        return [...filtered].sort((a, b) => {
            if (!sort.column) return 0;
            const aVal = a[sort.column];
            const bVal = b[sort.column];

            if (sort.column === 'tvlTotalAtomic') {
                if (aVal !== null && bVal !== null) {
                    const aBigInt = BigInt(aVal as string);
                    const bBigInt = BigInt(bVal as string);
                    if (aBigInt < bBigInt) return sort.asc ? -1 : 1;
                    if (aBigInt > bBigInt) return sort.asc ? 1 : -1;
                    return 0;
                }
            }

            if (aVal === null || bVal === null) return 0;
            if (aVal < bVal) return sort.asc ? -1 : 1;
            if (aVal > bVal) return sort.asc ? 1 : -1;
            return 0;
        });
    }, [tableData, sort, searchTerm]);

    const totalPages: number = Math.ceil(filteredAndSortedItems.length / perPage);
    const startRow: number = (page - 1) * perPage;
    const paginatedItems = filteredAndSortedItems.slice(startRow, startRow + perPage);

    const sortBy = (col: keyof TableItem): void => {
        setSort(prev => ({
            column: col,
            asc: prev.column === col ? !prev.asc : false,
        }));
    };

    const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSearchTerm(e.target.value);
        setPage(1);
    };

    return (
        <div
            className="overflow-hidden rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/[0.03]">
            <div
                className="flex flex-col gap-4 border-b border-gray-200 px-4 py-4 sm:flex-row sm:items-center sm:justify-between sm:px-5 dark:border-gray-800">
                <h3 className="text-lg font-semibold text-gray-800 dark:text-white/90">Operators</h3>
                <div className="relative">
                    <input
                        type="text"
                        value={searchTerm}
                        onChange={handleSearch}
                        placeholder="Search by Operator ID..."
                        className="h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 pl-4 pr-10 text-sm text-gray-800 shadow-theme-xs placeholder:text-gray-400 focus:border-brand-300 focus:outline-none focus:ring-3 focus:ring-brand-500/10 dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 dark:placeholder:text-white/30 sm:w-64"
                    />
                </div>
            </div>

            <div className="custom-scrollbar overflow-x-auto">
                <Table className="w-full min-w-[1000px] table-auto">
                    <TableHeader>
                        <TableRow className="border-b border-gray-200 dark:border-gray-800">
                            {headers.map((header) => (
                                <TableCell key={header.key} isHeader
                                           className={`p-4 ${header.sortable ? 'cursor-pointer' : ''}`}
                                >
                                    <div className="flex items-center gap-2"
                                         onClick={() => header.sortable && sortBy(header.key)}>
                                        <p className="text-xs font-medium text-gray-500 dark:text-gray-400">{header.label}</p>
                                        {header.sortable && (
                                            <div className="flex flex-col">
                                                <SortArrow direction="asc"
                                                           isActive={sort.column === header.key && sort.asc}/>
                                                <SortArrow direction="desc"
                                                           isActive={sort.column === header.key && !sort.asc}/>
                                            </div>
                                        )}
                                    </div>
                                </TableCell>
                            ))}
                        </TableRow>
                    </TableHeader>
                    <TableBody className="divide-y divide-gray-200 dark:divide-gray-800">
                        {paginatedItems.map((item) => (
                            <TableRow key={item.operatorId}
                                      className="transition hover:bg-gray-50 dark:hover:bg-gray-900">
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm font-medium text-gray-700 dark:text-gray-400">{shortenId(item.operatorId)}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-800 dark:text-white/90">{formatPowerOfTen(item.tvlTotalAtomic)}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-700 dark:text-white/90">{item.avsCount}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-700 dark:text-white/90">{item.strategyCount}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-700 dark:text-white/90">{item.slashingCount}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-700 dark:text-white/90">{formatTimestamp(item.lastSlashAt)}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-700 dark:text-white/90">{formatTimestamp(item.lastUpdateBlockTs)}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>
            <div
                className="flex flex-col items-center justify-between border-t border-gray-200 px-5 py-4 sm:flex-row dark:border-gray-800">
                <div className="pb-3 sm:pb-0">
                    <span className="block text-sm font-medium text-gray-500 dark:text-gray-400">
                        Showing <span
                        className="text-gray-800 dark:text-white/90">{filteredAndSortedItems.length > 0 ? startRow + 1 : 0}</span> to <span
                        className="text-gray-800 dark:text-white/90">{Math.min(startRow + perPage, filteredAndSortedItems.length)}</span> of <span
                        className="text-gray-800 dark:text-white/90">{filteredAndSortedItems.length}</span>
                    </span>
                </div>
                <div className="flex items-center gap-3">
                    <button onClick={() => setPage(p => p - 1)} disabled={page === 1}
                            className="rounded-lg border border-gray-300 px-3 py-2 text-sm font-medium text-gray-700 transition hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-900">Prev
                    </button>
                    <span
                        className="text-sm font-medium text-gray-700 dark:text-gray-400">Page {page} of {totalPages > 0 ? totalPages : 1}</span>
                    <button onClick={() => setPage(p => p + 1)} disabled={page === totalPages || totalPages === 0}
                            className="rounded-lg border border-gray-300 px-3 py-2 text-sm font-medium text-gray-700 transition hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-900">Next
                    </button>
                </div>
            </div>
        </div>
    );
};

export default OperatorsTable;