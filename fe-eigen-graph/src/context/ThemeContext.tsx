"use client";

import type React from "react";
import {createContext, useContext, useEffect} from "react";

type Theme = "light" | "dark";

type ThemeContextType = {
    theme: Theme;
};

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({
                                                                           children,
                                                                       }) => {
    const theme: Theme = "dark";

    useEffect(() => {
        document.documentElement.classList.add("dark");
    }, []);

    return (
        <ThemeContext.Provider value={{theme}}>
            {children}
        </ThemeContext.Provider>
    );
};

export const useTheme = () => {
    const context = useContext(ThemeContext);
    if (context === undefined) {
        throw new Error("useTheme must be used within a ThemeProvider");
    }
    return context;
};