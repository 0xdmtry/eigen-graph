"use client";

import React from "react";
import Image from "next/image";
import {formatScientific, formatPowerOfTen, formatCompact} from '@/utils/number-utils';

interface FileCardProps {
    symbol: string;
    name: string;
    icon: string;
    tvl: string;
    operators: number;
}

const TokenCard: React.FC<FileCardProps> = ({
                                                symbol,
                                                name,
                                                icon,
                                                tvl,
                                                operators,
                                            }) => {
    return (
        <div
            className="flex items-center justify-between rounded-2xl border border-gray-100 bg-white py-4 pl-4 pr-4 dark:border-gray-800 dark:bg-white/[0.03] xl:pr-5">
            <div className="flex items-center gap-4">
                <div className={`flex h-[52px] w-[52px] items-center justify-center rounded-xl`}>
                    {icon ? (<Image
                        src={`/images/tokens/${icon}.png`}
                        alt={name}
                        width={32}
                        height={32}
                    />) : (<span
                        className="text-xs font-bold text-gray-500 dark:text-gray-400">{symbol.charAt(0).toUpperCase()}</span>)}
                </div>
                <div>
                    <h4 className="mb-1 text-sm font-medium text-gray-800 dark:text-white/90">{symbol}</h4>
                    <span className="block text-sm text-gray-500 dark:text-gray-400">{name}</span>
                </div>
            </div>
            <div>
                <span
                    className="block mb-1 text-sm text-right text-gray-500 dark:text-gray-400">{formatPowerOfTen(tvl)}</span>
                <span
                    className="block text-sm text-right text-gray-500 dark:text-gray-400">{operators} operator{operators > 1 ? 's' : ''}</span>
            </div>
        </div>
    );
};

export default TokenCard;
