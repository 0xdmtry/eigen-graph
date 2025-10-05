"use client";
import React from "react";
import Image from "next/image";
import {formatPowerOfTen} from "@/utils/number-utils";

type Props = {
    id: string;
    symbol: string;
    name: string;
    icon: string;
    tvl: string;
    operators: number;
    isActive: boolean;
    onSelect: () => void;
    dragHandleRef?: React.Ref<HTMLButtonElement>;
    dragHandleAttributes?: React.HTMLAttributes<HTMLButtonElement>;
    dragHandleListeners?: React.HTMLAttributes<HTMLButtonElement>;
};

export default function TokenCardDnD({
                                         symbol,
                                         name,
                                         icon,
                                         tvl,
                                         operators,
                                         isActive,
                                         onSelect,
                                         dragHandleRef,
                                         dragHandleAttributes,
                                         dragHandleListeners,
                                     }: Props) {
    return (
        <div
            role="button"
            tabIndex={0}
            onClick={onSelect}
            onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                    e.preventDefault();
                    onSelect();
                }
            }}
            className={`w-full text-left rounded-2xl transition-all duration-200 focus:outline-none ${
                isActive ? "ring-2 ring-brand-500 ring-offset-2  ring-offset-gray-900" : "ring-0"
            }`}
        >
            <div
                className="flex h-full items-center justify-between rounded-2xl  py-4 pl-4 pr-4 dark:border-gray-800 dark:bg-white/[0.03] xl:pr-5">
                <div className="flex items-center gap-3 overflow-hidden">
                    <button
                        type="button"
                        aria-label="Drag"
                        ref={dragHandleRef}
                        {...dragHandleAttributes}
                        {...dragHandleListeners}
                        className="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-md   bg-transparent text-gray-500 hover:bg-gray-100 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-white/5"
                    >
                        <svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path
                                d="M6 5a1 1 0 110 2 1 1 0 010-2Zm0 4a1 1 0 110 2 1 1 0 010-2Zm0 4a1 1 0 110 2 1 1 0 010-2Zm8-8a1 1 0 110 2 1 1 0 010-2Zm0 4a1 1 0 110 2 1 1 0 010-2Zm0 4a1 1 0 110 2 1 1 0 010-2Z"/>
                        </svg>
                    </button>
                    <div className="flex h-[52px] w-[52px] flex-shrink-0 items-center justify-center rounded-xl">
                        {icon ? (
                            <Image src={`/images/tokens/${icon}.png`} alt={name} width={32} height={32}/>
                        ) : (
                            <span
                                className="text-xs font-bold text-gray-500 dark:text-gray-400">{symbol.charAt(0).toUpperCase()}</span>
                        )}
                    </div>
                    <div className="overflow-hidden">
                        <h4 className="mb-1 truncate text-sm font-medium text-gray-800 dark:text-white/90">{symbol}</h4>
                        <span className="block truncate text-sm text-gray-500 dark:text-gray-400">{name}</span>
                    </div>
                </div>
                <div className="flex-shrink-0">
          <span className="mb-1 block text-right text-sm text-gray-500 dark:text-gray-400">
            {tvl ? formatPowerOfTen(tvl) : "N/A"}
          </span>
                    <span className="block text-right text-sm text-gray-500 dark:text-gray-400">
            {operators} operator{operators === 1 ? "" : "s"}
          </span>
                </div>
            </div>
        </div>
    );
}
