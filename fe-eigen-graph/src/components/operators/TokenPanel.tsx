"use client";
import TokenCard from "./TokenCard";
import {TableItem} from "@/types/operators";
import React, {useMemo, useState} from "react";
import {TokenCardType} from "@/types/tokens";
import TokenAutocomplete from "@/components/operators/TokenAutocomplete";
import Badge from "@/components/ui/badge/Badge";
import {baseTokenCards} from "@/data/tokens";
import {useToken} from "@/context/TokenContext";

interface TokensProps {
    tokens: Record<string, TableItem[]>;
}

const createUpdatedTokenCards = (
    baseCards: TokenCardType[],
    tokenDataFromServer: Record<string, TableItem[]>
): TokenCardType[] => {
    return baseCards.map(card => {
        const dataForToken = tokenDataFromServer[card.symbol];
        if (!dataForToken || dataForToken.length === 0) return card;

        const operators = dataForToken.length;
        const totalTvl = dataForToken.reduce((sum, item) => {
            try {
                return sum + BigInt(item.tvlTotalAtomic);
            } catch {
                return sum;
            }
        }, BigInt(0));

        return {...card, operators, tvl: totalTvl.toString()};
    });
};

const TokenPanel: React.FC<TokensProps> = ({tokens}) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const {selectedTokenSymbol, setSelectedTokenSymbol} = useToken();

    const updatedTokenCards = useMemo(() => {
        return createUpdatedTokenCards(baseTokenCards, tokens);
    }, [tokens]);

    const CARDS_SHOWN_COLLAPSED = 6;
    const canExpand = updatedTokenCards.length > CARDS_SHOWN_COLLAPSED;

    return (
        <div className="rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/[0.03]">
            <div className="px-4 py-4 sm:pl-6 sm:pr-4">
                <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                    <h3 className="text-lg font-semibold text-gray-800 dark:text-white/90">
                        All Tokens <Badge variant="solid" color="dark">{baseTokenCards.length}</Badge>
                    </h3>
                    <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
                        <TokenAutocomplete/>
                    </div>
                </div>
            </div>
            <div className="border-t border-gray-100 dark:border-gray-800">
                <div
                    className={`overflow-hidden transition-[max-height] duration-700 ease-in-out ${isExpanded || !canExpand ? 'max-h-[4000px]' : 'max-h-[240px]'
                    }`}
                >
                    <div className="grid grid-cols-1 gap-4 p-4 sm:grid-cols-2 sm:gap-6 sm:p-6 xl:grid-cols-3">
                        {updatedTokenCards.map((item, i) => (
                            <TokenCard
                                key={i + 1}
                                symbol={item.symbol}
                                name={item.name}
                                icon={item.icon}
                                tvl={item.tvl}
                                operators={item.operators}
                                isActive={item.symbol.toUpperCase() === selectedTokenSymbol?.toUpperCase()}
                                onSelect={() => setSelectedTokenSymbol(item.symbol)}
                            />
                        ))}
                    </div>
                </div>

                {canExpand && (
                    <div className="border-t border-gray-100 p-2 dark:border-gray-800">
                        <button
                            onClick={() => setIsExpanded(!isExpanded)}
                            className="flex w-full items-center justify-center gap-2 rounded-lg py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-white/5"
                        >
                            <span>{isExpanded ? 'Show Less' : 'Show More'}</span>
                            <svg
                                className={`transform transition-transform duration-300 ${isExpanded ? 'rotate-180' : ''}`}
                                width="16" height="16" viewBox="0 0 16 16" fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path d="M4 6L8 10L12 6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"
                                      strokeLinejoin="round"/>
                            </svg>
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
}

export default TokenPanel;