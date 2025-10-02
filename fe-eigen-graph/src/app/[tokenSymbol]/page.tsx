import {notFound} from "next/navigation";
import {baseTokenCards} from "@/data/tokens";
import TokenPageClient from "@/components/page/TokenPageClient";

export default function Page({params}: { params: { tokenSymbol: string } }) {
    const symbol = params.tokenSymbol?.toUpperCase();
    const isValid = !!symbol && baseTokenCards.some(t => t.symbol.toUpperCase() === symbol);
    if (!isValid) notFound();
    return <TokenPageClient tokenSymbol={symbol}/>;
}
