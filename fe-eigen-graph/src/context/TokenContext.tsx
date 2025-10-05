"use client";

import React, {createContext, useContext, useState} from "react";

type TokenContextType = {
    selectedTokenSymbol: string;
    setSelectedTokenSymbol: (symbol: string) => void;
};

export const TokenContext = createContext<TokenContextType | undefined>(undefined);

export const TokenProvider: React.FC<{ children: React.ReactNode }> = ({children}) => {
    const [selectedTokenSymbol, setSelectedTokenSymbol] = useState("EIGEN");
    return (
        <TokenContext.Provider value={{selectedTokenSymbol, setSelectedTokenSymbol}}>
            {children}
        </TokenContext.Provider>
    );
};

export const useToken = () => {
    const ctx = useContext(TokenContext);
    if (!ctx) throw new Error("useToken must be used within a TokenProvider");
    return ctx;
};
