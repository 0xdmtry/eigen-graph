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
    return new Date(ts * 1000).toLocaleString();
};

const OperatorsTable: React.FC<OperatorsTableProps> = ({tableData}) => {
    const [sort, setSort] = useState<{ column: keyof TableItem | ""; asc: boolean }>({
        column: "tvlTotalAtomic",
        asc: false,
    });
    const [page, setPage] = useState<number>(1);
    const perPage: number = 10;

    const headers: { key: keyof TableItem; label: string }[] = [
        {key: "operatorId", label: "Operator ID"},
        {key: "tvlTotalAtomic", label: "TVL"},
        {key: "avsCount", label: "# AVSs"},
        {key: "strategyCount", label: "# Strategies"},
        {key: "slashingCount", label: "Slashing Count"},
        {key: "lastSlashAt", label: "Last Slash At"},
        {key: "lastUpdateBlockTs", label: "Last Update"},
    ];

    console.log("tableData", tableData);

    const sortedAndPaginatedItems = useMemo(() => {
        const sorted = [...tableData].sort((a, b) => {
            if (!sort.column) return 0;

            const aVal = a[sort.column];
            const bVal = b[sort.column];

            if (aVal === null || bVal === null) return 0;
            if (aVal < bVal) return sort.asc ? -1 : 1;
            if (aVal > bVal) return sort.asc ? 1 : -1;
            return 0;
        });

        const startRow = (page - 1) * perPage;
        const endRow = page * perPage;
        return sorted.slice(startRow, endRow);

    }, [tableData, sort, page, perPage]);


    const totalPages: number = Math.ceil(tableData.length / perPage);
    const startRow: number = (page - 1) * perPage;
    const endRow: number = page * perPage;

    const sortBy = (col: keyof TableItem): void => {
        setSort((prev) => ({
            column: col,
            asc: prev.column === col ? !prev.asc : false, // Default to descending for new columns
        }));
    };

    const prevPage = (): void => {
        if (page > 1) setPage(page - 1);
    };

    const nextPage = (): void => {
        if (page < totalPages) setPage(page + 1);
    };


    return (
        <div
            className="overflow-hidden rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/[0.03]">
            <div
                className="flex flex-col gap-4 border-b border-gray-200 px-4 py-4 sm:px-5 lg:flex-row lg:items-center lg:justify-between dark:border-gray-800">
                <h3 className="text-lg font-semibold text-gray-800 dark:text-white/90">Operators</h3>
            </div>

            <div className="custom-scrollbar overflow-x-auto">
                <Table className="w-full min-w-[1000px] table-auto">
                    <TableHeader>
                        <TableRow className="border-b border-gray-200 dark:divide-gray-800 dark:border-gray-800">
                            {headers.map((header) => (
                                <TableCell key={header.key} isHeader className="p-4 cursor-pointer"
                                >
                                    <div className="flex items-center gap-2">
                                        <p className="text-xs font-medium text-gray-500 dark:text-gray-400">{header.label}</p>
                                    </div>
                                </TableCell>
                            ))}
                        </TableRow>
                    </TableHeader>
                    <TableBody className="divide-y divide-gray-200 dark:divide-gray-800">
                        {sortedAndPaginatedItems.map((item) => (
                            <TableRow key={item.operatorId}
                                      className="transition hover:bg-gray-50 dark:hover:bg-gray-900">
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm font-medium text-gray-700 dark:text-gray-400">{shortenId(item.operatorId)}</TableCell>
                                <TableCell
                                    className="p-4 whitespace-nowrap text-sm text-gray-800 dark:text-white/90">{item.tvlTotalAtomic}</TableCell>
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
                className="flex items-center flex-col sm:flex-row justify-between border-t border-gray-200 px-5 py-4 dark:border-gray-800">
                <div className="pb-3 sm:pb-0">
                    <span className="block text-sm font-medium text-gray-500 dark:text-gray-400">
                      Showing{" "}
                        <span className="text-gray-800 dark:text-white/90">{startRow + 1}</span>{" "}
                        to{" "}<span
                        className="text-gray-800 dark:text-white/90">{Math.min(endRow, tableData.length)}</span>{" "}
                        of{" "}<span className="text-gray-800 dark:text-white/90">{tableData.length}</span>
                    </span>
                </div>
                <div className="flex items-center gap-2">
                    <button onClick={prevPage} disabled={page === 1}>Prev</button>
                    <span className="text-sm">Page {page} of {totalPages}</span>
                    <button onClick={nextPage} disabled={page === totalPages}>Next
                    </button>
                </div>
            </div>
        </div>
    );
};

export default OperatorsTable;