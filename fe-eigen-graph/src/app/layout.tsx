import {Outfit} from "next/font/google";
import "./globals.css";
import {ThemeProvider} from "@/context/ThemeContext";
import {TokenProvider} from "@/context/TokenContext";

const outfit = Outfit({
    subsets: ["latin"],
});
export default function RootLayout({
                                       children,
                                   }: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
        <body className={`${outfit.className} dark:bg-gray-900`}>
        <ThemeProvider>
            <TokenProvider>{children}</TokenProvider>
        </ThemeProvider>
        </body>
        </html>
    );
}
