"use client";

import React from "react";
import Image from "next/image";
import {formatPowerOfTen} from '@/utils/number-utils';

interface TokenCardProps {
    symbol: string;
    name: string;
    icon: string;
    tvl: string;
    operators: number;
    isActive: boolean;
    onSelect: () => void;
}

const TokenCard: React.FC<TokenCardProps> = ({
                                                 symbol,
                                                 name,
                                                 icon,
                                                 tvl,
                                                 operators,
                                                 isActive,
                                                 onSelect,
                                             }) => {
    return (
        <button
            onClick={onSelect}
            className={`w-full text-left rounded-2xl transition-all duration-200 focus:outline-none ${
                isActive
                    ? 'ring-2 ring-brand-500 ring-offset-2 ring-offset-white dark:ring-offset-gray-900'
                    : 'ring-0'
            }`}
        >
            <div
                className="flex h-full items-center justify-between rounded-2xl border border-gray-100 bg-white py-4 pl-4 pr-4 dark:border-gray-800 dark:bg-white/[0.03] xl:pr-5">
                <div className="flex items-center gap-4 overflow-hidden">
                    <div className={`flex h-[52px] w-[52px] flex-shrink-0  items-center justify-center rounded-xl`}>
                        {icon ? (<Image
                            src={`/images/tokens/${icon}.png`}
                            alt={name}
                            width={32}
                            height={32}
                        />) : (<span
                            className="text-xs font-bold text-gray-500 dark:text-gray-400">{symbol.charAt(0).toUpperCase()}</span>)}
                    </div>
                    <div className="overflow-hidden">
                        <h4 className="truncate mb-1 text-sm font-medium text-gray-800 dark:text-white/90">{symbol}</h4>
                        <span className="truncate block text-sm text-gray-500 dark:text-gray-400">{name}</span>
                    </div>
                </div>
                <div className="flex-shrink-0">
                <span
                    className="block mb-1 text-sm text-right text-gray-500 dark:text-gray-400">{tvl ? formatPowerOfTen(tvl) : 'N/A'}</span>
                    <span
                        className="block text-sm text-right text-gray-500 dark:text-gray-400">{operators} operator{operators === 1 ? '' : 's'}</span>
                </div>
            </div>
        </button>
    );
};

export default TokenCard;
