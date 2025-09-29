"use client";

import type React from "react";
import {createContext, useContext} from "react";
import {useParams, useRouter} from "next/navigation";

type TokenContextType = {
    selectedTokenSymbol: string | null;
    setSelectedTokenSymbol: (symbol: string) => void;
};

const TokenContext = createContext<TokenContextType | undefined>(undefined);

export const TokenProvider: React.FC<{ children: React.ReactNode }> = ({children}) => {
    const router = useRouter();
    const params = useParams();

    const currentSymbol = typeof params.tokenSymbol === 'string'
        ? params.tokenSymbol.toUpperCase()
        : null;

    const setSelectedTokenSymbol = (symbol: string) => {
        router.push(`/${symbol.toLowerCase()}`, {scroll: false});
    };

    return (
        <TokenContext.Provider value={{selectedTokenSymbol: currentSymbol, setSelectedTokenSymbol}}>
            {children}
        </TokenContext.Provider>
    );
};

export const useToken = () => {
    const context = useContext(TokenContext);
    if (context === undefined) {
        throw new Error("useToken must be used within a TokenProvider");
    }
    return context;
};