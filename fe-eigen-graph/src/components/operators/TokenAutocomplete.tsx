"use client";

import React, {useState, useEffect, useRef} from 'react';
import Image from 'next/image';
import {baseTokenCards} from "@/data/tokens";
import {TokenCardType} from "@/types/tokens";

const TokenAutocomplete: React.FC = () => {
    const [inputValue, setInputValue] = useState('');
    const [suggestions, setSuggestions] = useState<TokenCardType[]>([]);
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);
    const containerRef = useRef<HTMLDivElement>(null);

    const filterTokens = (query: string) => {
        if (!query) {
            return baseTokenCards;
        }
        const lowerCaseQuery = query.toLowerCase();
        return baseTokenCards.filter(
            token =>
                token.symbol.toLowerCase().includes(lowerCaseQuery) ||
                token.name.toLowerCase().includes(lowerCaseQuery)
        );
    };

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
                setIsDropdownOpen(false);
            }
        };
        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);

    const handleFocus = () => {
        setSuggestions(baseTokenCards);
        setIsDropdownOpen(true);
    };

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const query = e.target.value;
        setInputValue(query);
        setSuggestions(filterTokens(query));
        setIsDropdownOpen(true);
    };

    const handleSuggestionClick = (token: TokenCardType) => {
        setInputValue(token.name);
        setIsDropdownOpen(false);
    };

    return (
        <div className="relative w-full max-w-xs" ref={containerRef}>
            <div className="relative">
                <button
                    className="absolute text-gray-500 -translate-y-1/2 left-4 top-1/2 dark:text-gray-400">
                    <svg
                        width="20"
                        height="20"
                        viewBox="0 0 20 20"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            fillRule="evenodd"
                            clipRule="evenodd"
                            d="M3.04199 9.37363C3.04199 5.87693 5.87735 3.04199 9.37533 3.04199C12.8733 3.04199 15.7087 5.87693 15.7087 9.37363C15.7087 12.8703 12.8733 15.7053 9.37533 15.7053C5.87735 15.7053 3.04199 12.8703 3.04199 9.37363ZM9.37533 1.54199C5.04926 1.54199 1.54199 5.04817 1.54199 9.37363C1.54199 13.6991 5.04926 17.2053 9.37533 17.2053C11.2676 17.2053 13.0032 16.5344 14.3572 15.4176L17.1773 18.238C17.4702 18.5309 17.945 18.5309 18.2379 18.238C18.5308 17.9451 18.5309 17.4703 18.238 17.1773L15.4182 14.3573C16.5367 13.0033 17.2087 11.2669 17.2087 9.37363C17.2087 5.04817 13.7014 1.54199 9.37533 1.54199Z"
                            fill="currentColor"
                        />
                    </svg>
                </button>
                <input
                    type="text"
                    value={inputValue}
                    onChange={handleChange}
                    onFocus={handleFocus}
                    placeholder="Select Token..."
                    className="h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 px-12 text-sm text-gray-800 shadow-theme-xs placeholder:text-gray-400 focus:border-brand-300 focus:outline-none focus:ring-3 focus:ring-brand-500/10 dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 dark:placeholder:text-white/30 dark:focus:border-brand-800"
                />
                <button
                    className="absolute items-center justify-center w-full gap-2 px-4 py-3 text-sm font-medium text-white rounded-lg bg-brand-500 shadow-theme-xs hover:bg-brand-600 sm:w-auto">
                    <svg
                        className="duration-200 ease-in-out stroke-current rotate-270"
                        width="20"
                        height="20"
                        viewBox="0 0 20 20"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M4.79199 7.396L10.0003 12.6043L15.2087 7.396"
                            stroke=""
                            strokeWidth="1.5"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                        />
                    </svg>
                </button>
            </div>
            {isDropdownOpen && suggestions.length > 0 && (
                <ul className="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-lg border border-gray-200 bg-white p-2 shadow-theme-lg dark:border-gray-800 dark:bg-[#1E2635]">
                    {suggestions.map((token) => (
                        <li
                            key={token.symbol}
                            onClick={() => handleSuggestionClick(token)}
                            className="flex cursor-pointer items-center gap-3 rounded-md p-2 text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-white/5"
                        >
                            <div
                                className="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-700">
                                {token.icon ? (
                                    <Image
                                        src={`/images/tokens/${token.icon}.png`}
                                        alt={token.name}
                                        width={24}
                                        height={24}
                                    />
                                ) : (
                                    <span className="text-xs font-bold text-gray-500 dark:text-gray-400">
                                        {token.symbol.charAt(0)}
                                    </span>
                                )}
                            </div>
                            <div className="flex flex-col overflow-hidden">
                                <span
                                    className="truncate font-medium text-gray-800 dark:text-white/90">{token.symbol}</span>
                                <span className="truncate text-xs text-gray-500 dark:text-gray-400">{token.name}</span>
                            </div>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};

export default TokenAutocomplete;